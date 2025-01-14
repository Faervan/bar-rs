use std::{collections::HashMap, time::Duration};

use bar_rs_derive::Builder;
use iced::{futures::SinkExt, stream, widget::text, Subscription};
use tokio::{fs, io, runtime, select, sync::mpsc, task, time::sleep};
use udev::Device;

use crate::{
    config::{
        anchor::BarAnchor,
        module_config::{LocalModuleConfig, ModuleConfigOverride},
    },
    fill::FillExt,
    Message, NERD_FONT,
};

use super::Module;

#[derive(Debug, Default, Builder)]
pub struct BatteryMod {
    stats: BatteryStats,
    cfg_override: ModuleConfigOverride,
}

#[derive(Debug, Default)]
struct BatteryStats {
    capacity: u16,
    hours: u16,
    minutes: u16,
    power_now_is_zero: bool,
    icon: &'static str,
}

impl Module for BatteryMod {
    fn name(&self) -> String {
        "battery".to_string()
    }

    fn view(&self, config: &LocalModuleConfig, anchor: &BarAnchor) -> iced::Element<Message> {
        list![
            anchor,
            text!("{}", self.stats.icon)
                .fill(anchor)
                .color(self.cfg_override.icon_color.unwrap_or(config.icon_color))
                .size(self.cfg_override.icon_size.unwrap_or(config.icon_size))
                .font(NERD_FONT),
            match self.stats.power_now_is_zero {
                true => text!["{}% (unknown)", self.stats.capacity],
                false => text![
                    "{}% ({}h {}min left)",
                    self.stats.capacity,
                    self.stats.hours,
                    self.stats.minutes
                ],
            }
            .fill(anchor)
            .color(self.cfg_override.text_color.unwrap_or(config.text_color))
            .size(self.cfg_override.font_size.unwrap_or(config.font_size))
        ]
        .spacing(self.cfg_override.spacing.unwrap_or(config.spacing))
        .into()
    }

    fn read_config(&mut self, config: &HashMap<String, Option<String>>) {
        self.cfg_override = config.into();
    }

    fn subscription(&self) -> Option<iced::Subscription<Message>> {
        Some(Subscription::run(|| {
            let (sx, mut rx) = mpsc::channel(10);
            std::thread::spawn(move || {
                let local = task::LocalSet::new();
                let runtime = runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();

                runtime.block_on(local.run_until(async move {
                    task::spawn_local(async move {
                        let socket = udev::MonitorBuilder::new()
                            .and_then(|b| b.match_subsystem_devtype("power_supply", "power_supply"))
                            .and_then(|b| b.listen())
                            .expect("Failed to build udev MonitorBuilder");

                        loop {
                            let Some(event) = socket.iter().next() else {
                                sleep(Duration::from_millis(10)).await;
                                continue;
                            };

                            if event.sysname() != "AC" {
                                continue;
                            }
                            sleep(Duration::from_secs(1)).await;
                            sx.send(()).await.expect("mpsc channel closed");
                        }
                    })
                    .await
                    .unwrap();
                }));
            });

            stream::channel(1, |mut sender| async move {
                tokio::spawn(async move {
                    loop {
                        let stats = get_stats().await.unwrap();
                        sender
                            .send(Message::update(move |reg| {
                                reg.get_module_mut::<BatteryMod>().stats = stats
                            }))
                            .await
                            .unwrap_or_else(|err| {
                                eprintln!("Trying to send battery_stats failed with err: {err}");
                            });
                        select! {
                            _ = sleep(Duration::from_secs(30)) => {}
                            _ = rx.recv() => {}
                        }
                    }
                });
            })
        }))
    }
}

#[derive(Default, Debug)]
struct Battery {
    energy_now: f32,
    energy_full: f32,
    power_now: f32,
    voltage_now: f32,
    status: bool,
}

impl From<&Device> for Battery {
    fn from(device: &Device) -> Self {
        Battery {
            energy_now: get_property(device, "POWER_SUPPLY_ENERGY_NOW")
                .parse()
                .unwrap_or(0.),
            energy_full: get_property(device, "POWER_SUPPLY_ENERGY_FULL")
                .parse()
                .unwrap_or(0.),
            power_now: get_property(device, "POWER_SUPPLY_POWER_NOW")
                .parse()
                .unwrap_or(0.),
            voltage_now: get_property(device, "POWER_SUPPLY_VOLTAGE_NOW")
                .parse()
                .unwrap_or(0.),
            status: matches!(get_property(device, "POWER_SUPPLY_STATUS"), "Charging"),
        }
    }
}

fn get_property<'a>(device: &'a Device, property: &'static str) -> &'a str {
    device
        .property_value(property)
        .and_then(|v| v.to_str())
        .unwrap_or("")
}

async fn get_stats() -> Result<BatteryStats, io::Error> {
    let mut entries = fs::read_dir("/sys/class/power_supply").await?;
    let mut batteries = vec![];
    while let Ok(Some(dev_name)) = entries.next_entry().await {
        if let Ok(dev_type) =
            fs::read_to_string(&format!("{}/type", dev_name.path().to_string_lossy())).await
        {
            if dev_type.trim() == "Battery" {
                batteries.push(dev_name.path());
            }
        }
    }
    let batteries = batteries.iter().fold(vec![], |mut acc, bat| {
        let Ok(device) = Device::from_syspath(bat) else {
            eprintln!(
                "Battery {} could not be turned into a udev Device",
                bat.to_string_lossy()
            );
            return acc;
        };

        acc.push(Battery::from(&device));
        acc
    });

    let energy_now = batteries.iter().fold(0., |mut acc, bat| {
        acc += bat.energy_now;
        acc
    });
    let energy_full = batteries.iter().fold(0., |mut acc, bat| {
        acc += bat.energy_full;
        acc
    });
    let (power_now, voltage_now) =
        batteries
            .iter()
            .filter(|bat| bat.power_now != 0.)
            .fold((0., 0.), |mut acc, bat| {
                acc.0 += bat.power_now;
                acc.1 += bat.voltage_now;
                acc
            });

    let capacity = (100. / energy_full * energy_now).round() as u16;
    let charging = batteries.iter().any(|bat| bat.status);
    let time_remaining = match charging {
        true => {
            (energy_full - energy_now)
                / 1000000.
                / ((power_now / 1000000.) * (voltage_now / 1000000.))
                * 12.55
        }
        false => energy_now / power_now,
    };

    Ok(BatteryStats {
        capacity,
        hours: time_remaining.floor() as u16,
        minutes: ((time_remaining - time_remaining.floor()) * 60.) as u16,
        power_now_is_zero: power_now == 0.,
        icon: match charging {
            false => match capacity {
                n if n >= 80 => "󱊣",
                n if n >= 60 => "󱊢",
                n if n >= 25 => "󱊡",
                _ => "󰂎",
            },
            true => match capacity {
                n if n >= 80 => "󱊦 ",
                n if n >= 60 => "󱊥 ",
                n if n >= 25 => "󱊤 ",
                _ => "󰢟",
            },
        },
    })
}

/*
    How upower calculates remaining time (upower/src/up-daemon.c):
    /* calculate a quick and dirty time remaining value
     * NOTE: Keep in sync with per-battery estimation code! */
    if (energy_rate_total > 0) {
        if (state_total == UP_DEVICE_STATE_DISCHARGING)
            time_to_empty_total = SECONDS_PER_HOUR * (energy_total / energy_rate_total);
        else if (state_total == UP_DEVICE_STATE_CHARGING)
            time_to_full_total = SECONDS_PER_HOUR * ((energy_full_total - energy_total) / energy_rate_total);
    }
*/
