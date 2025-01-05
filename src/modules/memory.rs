use std::{collections::HashMap, process::Command};

use bar_rs_derive::Builder;
use iced::{
    widget::{row, text},
    Length::Fill,
};

use crate::{
    config::module_config::{LocalModuleConfig, ModuleConfigOverride},
    Message, NERD_FONT,
};

use super::Module;

#[derive(Debug, Default, Builder)]
pub struct MemoryMod {
    cfg_override: ModuleConfigOverride,
}

impl Module for MemoryMod {
    fn id(&self) -> String {
        "memory".to_string()
    }

    fn view(&self, config: &LocalModuleConfig) -> iced::Element<Message> {
        let usage = Command::new("sh")
            .arg("-c")
            .arg("free | grep Mem | awk '{printf \"%.0f\", $3/$2 * 100.0}'")
            .output()
            .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
            .unwrap_or_else(|e| {
                eprintln!("Failed to get memory usage. err: {e}");
                "0".to_string()
            })
            .parse()
            .unwrap_or_else(|e| {
                eprintln!("Failed to parse memory usage (output from free), e: {e}");
                999
            });

        row![
            text!("󰍛")
                .center()
                .height(Fill)
                .size(self.cfg_override.icon_size.unwrap_or(config.icon_size))
                .color(self.cfg_override.icon_color.unwrap_or(config.icon_color))
                .font(NERD_FONT),
            text!["{}%", usage]
                .center()
                .height(Fill)
                .size(self.cfg_override.font_size.unwrap_or(config.font_size))
                .color(self.cfg_override.text_color.unwrap_or(config.text_color)),
        ]
        .spacing(10)
        .into()
    }

    fn read_config(&mut self, config: &HashMap<String, Option<String>>) {
        self.cfg_override = config.into();
    }
}
