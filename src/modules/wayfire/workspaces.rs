use std::{any::TypeId, collections::HashMap};

use bar_rs_derive::Builder;
use iced::widget::text;

use crate::{
    config::{
        anchor::BarAnchor,
        module_config::{LocalModuleConfig, ModuleConfigOverride},
    },
    fill::FillExt,
    listeners::wayfire::WayfireListener,
    modules::Module,
    Message, NERD_FONT,
};

/// I am unaware of a IPC method that gives a list of currently active workspaces (the ones with an
/// open window), and this is generally tricky here, since all workspaces of a wset grid are active
/// in a way. It would probably be posible to calculate the workspace of each active window
/// manually, but I'm too lazy to do that atm.

#[derive(Debug, Default, Builder)]
pub struct WayfireWorkspaceMod {
    pub active: (i64, i64),
    icons: HashMap<(i64, i64), String>,
    cfg_override: ModuleConfigOverride,
}

impl Module for WayfireWorkspaceMod {
    fn id(&self) -> String {
        "wayfire.workspaces".to_string()
    }

    fn view(&self, config: &LocalModuleConfig, anchor: &BarAnchor) -> iced::Element<Message> {
        text!(
            "{}",
            self.icons
                .get(&self.active)
                .unwrap_or(&format!("{}/{}", self.active.0, self.active.1))
        )
        .fill(anchor)
        .size(self.cfg_override.icon_size.unwrap_or(config.icon_size))
        .color(self.cfg_override.icon_color.unwrap_or(config.icon_color))
        .font(NERD_FONT)
        .into()
    }

    fn requires(&self) -> Vec<std::any::TypeId> {
        vec![TypeId::of::<WayfireListener>()]
    }

    fn read_config(&mut self, config: &HashMap<String, Option<String>>) {
        config.iter().for_each(|(key, val)| {
            if let Some(key) = key
                .strip_prefix('(')
                .and_then(|v| v.strip_suffix(')'))
                .and_then(|v| {
                    let [x, y] = v.split(',').map(|item| item.trim()).collect::<Vec<&str>>()[..]
                    else {
                        return None;
                    };
                    x.parse().and_then(|x| y.parse().map(|y| (x, y))).ok()
                })
            {
                self.icons.insert(key, val.clone().unwrap_or(String::new()));
            }
        });
        self.cfg_override = config.into();
    }
}