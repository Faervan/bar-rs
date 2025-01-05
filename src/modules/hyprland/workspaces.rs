use std::{any::TypeId, collections::HashMap, time::Duration};

use bar_rs_derive::Builder;
use hyprland::{
    data::{Workspace, Workspaces},
    shared::{HyprData, HyprDataActive, HyprDataVec},
};
use iced::{
    widget::{
        row,
        text::{Rich, Span},
    },
    Background, Border, Color,
    Length::Fill,
    Padding,
};
use tokio::time::sleep;

use crate::{
    config::{
        module_config::{LocalModuleConfig, ModuleConfigOverride},
        parse::StringExt,
    },
    listeners::hyprland::HyprListener,
    modules::{require_listener, Module},
    Message, NERD_FONT,
};

#[derive(Debug, Builder)]
pub struct HyprWorkspaceMod {
    pub active: usize,
    // (Name, Fullscreen state)
    pub open: Vec<(String, bool)>,
    cfg_override: ModuleConfigOverride,
    active_color: Color,
    active_background: Color,
}

impl Default for HyprWorkspaceMod {
    fn default() -> Self {
        Self {
            active_color: Color::BLACK,
            active_background: Color::WHITE.scale_alpha(0.5),
            active: 0,
            open: vec![],
            cfg_override: ModuleConfigOverride::default(),
        }
    }
}

impl Module for HyprWorkspaceMod {
    fn id(&self) -> String {
        "hyprland.workspaces".to_string()
    }

    fn view(&self, config: &LocalModuleConfig) -> iced::Element<Message> {
        row(self.open.iter().enumerate().map(|(id, (ws, _))| {
            let mut span = Span::new(ws)
                .size(self.cfg_override.icon_size.unwrap_or(config.icon_size))
                .color(self.cfg_override.icon_color.unwrap_or(config.icon_color))
                .padding(Padding {
                    top: -3.,
                    bottom: 0.,
                    right: 10.,
                    left: 5.,
                })
                .font(NERD_FONT);
            if id == self.active {
                span = span
                    .background(Background::Color(self.active_background))
                    .border(Border::default().rounded(8))
                    .color(self.active_color);
            }
            Rich::with_spans([span]).center().height(Fill).into()
        }))
        .spacing(15)
        .into()
    }

    fn requires(&self) -> Vec<TypeId> {
        vec![require_listener::<HyprListener>()]
    }

    fn read_config(&mut self, config: &HashMap<String, Option<String>>) {
        self.cfg_override = config.into();
        if let Some(color) = config.get("active_color").and_then(|v| v.into_color()) {
            self.active_color = color;
        }
        if let Some(color) = config.get("active_background").and_then(|v| v.into_color()) {
            self.active_background = color;
        }
    }
}

pub async fn get_workspaces(active: Option<i32>) -> (usize, Vec<(String, bool)>) {
    // Sleep a bit, to reduce the probability that a nonexisting ws is still reported active
    sleep(Duration::from_millis(10)).await;
    let Ok(workspaces) = Workspaces::get_async().await else {
        eprintln!("[hyprland.workspaces] Failed to get Workspaces!");
        return (0, vec![]);
    };
    let mut open = workspaces.to_vec();
    open.sort_by(|a, b| a.id.cmp(&b.id));
    (
        open.iter()
            .position(|ws| {
                ws.id
                    == active
                        .unwrap_or_else(|| Workspace::get_active().map(|ws| ws.id).unwrap_or(0))
            })
            .unwrap_or(0),
        open.into_iter()
            .map(|ws| (ws.name, ws.fullscreen))
            .collect(),
    )
}