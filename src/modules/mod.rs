use std::{any::Any, fmt::Debug};

use crate::subscription::Subscription;

pub trait Module: Any + Debug {
    fn subscription(&self) -> Subscription {
        Subscription::none()
    }
}
