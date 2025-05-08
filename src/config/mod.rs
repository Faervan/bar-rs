use std::path::PathBuf;

use insets::Insets;
use serde::Deserialize;
use types::{BarAnchor, KbFocus};

mod insets;
/// All the custom config types and their parsing implementations
mod types;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub path: String,
    pub monitor: Option<String>,
    pub anchor: BarAnchor,
    pub kb_focus: KbFocus,
    pub reload_interval: f64,
    width: Option<u32>,
    height: Option<u32>,
    pub margin: Insets<i32>,
    pub hot_reloading: bool,
    pub hard_reloading: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: format!(
                "{}/.config/bar-rs/bar-rs.toml",
                std::env::var("HOME").unwrap()
            ),
            monitor: None,
            anchor: BarAnchor::Top,
            kb_focus: KbFocus::None,
            reload_interval: 3.,
            width: None,
            height: None,
            margin: Insets::all(0),
            hot_reloading: true,
            hard_reloading: false,
        }
    }
}

impl Config {
    pub fn load(path: Option<&String>) -> anyhow::Result<Self> {
        let default = Config::default();
        let path = PathBuf::from(path.unwrap_or(&default.path));
        let config = config::Config::builder()
            .set_default("path", default.path)?
            .set_default("anchor", &default.anchor)?
            .set_default("kb_focus", &default.kb_focus)?
            .set_default("reload_interval", default.reload_interval)?
            .set_default("hot_reloading", default.hot_reloading)?
            .set_default("hard_reloading", default.hard_reloading)?
            .add_source(config::File::from(path))
            .build()?;
        Ok(config.try_deserialize()?)
    }

    pub fn exclusive_zone(&self) -> i32 {
        (match self.anchor {
            BarAnchor::Left | BarAnchor::Right => self.width.unwrap_or(30),
            BarAnchor::Top | BarAnchor::Bottom => self.height.unwrap_or(30),
        }) as i32
    }

    /// Determine the size to pass to `get_layer_surface`
    pub fn dimension(&self, x: u32, y: u32) -> Option<(Option<u32>, Option<u32>)> {
        let [width, height] = match self.anchor.is_vertical() {
            true => [self.width.unwrap_or(50), self.height.unwrap_or(y)],
            false => [self.width.unwrap_or(x), self.height.unwrap_or(30)],
        };
        Some((Some(width), Some(height)))
    }
}
