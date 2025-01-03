use std::{fs::File, io::{self, BufRead, BufReader, ErrorKind}, num, time::Duration};

use bar_rs_derive::Builder;
use iced::{futures::SinkExt, stream, widget::{row, text}, Length::Fill, Subscription};
use tokio::time::sleep;

use crate::{Message, NERD_FONT};

use super::Module;

#[derive(Debug, Default, Builder)]
pub struct CpuMod {
    usage: usize
}

impl Module for CpuMod {
    fn id(&self) -> String {
        "cpu".to_string()
    }

    fn view(&self) -> iced::Element<Message> {
        row![
            text!("󰻠")
                .center().height(Fill).size(20).font(NERD_FONT),
            text![
                "{}%", self.usage
            ].center().height(Fill),
        ].spacing(10).into()
    }

    fn subscription(&self) -> Option<iced::Subscription<Message>> {
        Some(Subscription::run(||
            stream::channel(1, |mut sender| async move {
                loop {
                    let (mut active, mut total) = (vec![], vec![]);
                    for _ in 0..3 {
                        sleep(Duration::from_millis(1000 / 3)).await;
                        let (a, t) = read_stats()
                            .unwrap_or_else(|e|
                                panic!("Failed to read cpu stats from /proc/stat ... err: {e:?}"));
                        active.push(a);
                        total.push(t);
                    }

                    let delta_active = (active[1] - active[0]) + (active[2] - active[1]);
                    let delta_total = (total[1] - total[0]) + (total[2] - total[1]);

                    let average = match delta_total == 0 {
                        true => 0.,
                        false => (delta_active as f32 / delta_total as f32) * 100.0
                    };

                    sender.send(Message::update(Box::new(
                            move |reg| reg.get_module_mut::<CpuMod>().usage = average as usize
                        )))
                        .await
                        .unwrap_or_else(|err| {
                            eprintln!("Trying to send cpu_usage failed with err: {err}");
                        });

                    sleep(Duration::from_secs(2)).await;
                }
            })
        ))
    }
}

fn read_stats() -> Result<(u32, u32), ReadError> {
    let file = File::open("/proc/stat")?;
    let reader = BufReader::new(file);
    let line = reader.lines().next()
        .ok_or(io::Error::new(ErrorKind::UnexpectedEof, "shit"))??;

    let parts: Vec<&str> = line.split_whitespace().collect();
    let user: u32 = parts[1].parse()?;
    let nice: u32 = parts[2].parse()?;
    let system: u32 = parts[3].parse()?;
    let idle: u32 = parts[4].parse()?;
    let iowait: u32 = parts[5].parse()?;
    let irq: u32 = parts[6].parse()?;
    let softirq: u32 = parts[7].parse()?;

    let active_time = user + nice + system + irq + softirq;
    let total_time = active_time + idle + iowait;

    Ok((active_time, total_time))
}

#[allow(dead_code)]
#[derive(Debug)]
enum ReadError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for ReadError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<num::ParseIntError> for ReadError {
    fn from(value: num::ParseIntError) -> Self {
        Self::ParseError(value)
    }
}
