use config::ValueKind;
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use smithay_client_toolkit::shell::wlr_layer::{Anchor, KeyboardInteractivity};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BarAnchor {
    Top,
    Bottom,
    Left,
    Right,
}

impl From<&BarAnchor> for ValueKind {
    fn from(value: &BarAnchor) -> Self {
        let s = to_variant_name(value).unwrap();
        ValueKind::String(s.to_owned())
    }
}

impl From<&BarAnchor> for Anchor {
    fn from(value: &BarAnchor) -> Self {
        match value {
            BarAnchor::Top => Anchor::TOP,
            BarAnchor::Bottom => Anchor::BOTTOM,
            BarAnchor::Left => Anchor::LEFT,
            BarAnchor::Right => Anchor::RIGHT,
        }
    }
}

impl BarAnchor {
    pub fn is_vertical(&self) -> bool {
        match self {
            BarAnchor::Top | BarAnchor::Bottom => false,
            _ => true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KbFocus {
    None,
    Exclusive,
    OnDemand,
}

impl From<&KbFocus> for ValueKind {
    fn from(value: &KbFocus) -> Self {
        let s = to_variant_name(value).unwrap();
        ValueKind::String(s.to_owned())
    }
}

impl From<&KbFocus> for KeyboardInteractivity {
    fn from(value: &KbFocus) -> Self {
        match value {
            KbFocus::None => KeyboardInteractivity::None,
            KbFocus::Exclusive => KeyboardInteractivity::Exclusive,
            KbFocus::OnDemand => KeyboardInteractivity::OnDemand,
        }
    }
}
