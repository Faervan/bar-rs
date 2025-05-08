use std::str::FromStr;

use iced::runtime::platform_specific::wayland::layer_surface::IcedMargin;
use serde::Deserialize;

#[derive(Debug)]
pub struct Insets<T: FromStr> {
    t: T,
    b: T,
    l: T,
    r: T,
}

impl<T: FromStr + Copy> Insets<T> {
    pub fn all(v: T) -> Self {
        Insets {
            t: v,
            b: v,
            l: v,
            r: v,
        }
    }
}

impl From<&Insets<i32>> for IcedMargin {
    fn from(p: &Insets<i32>) -> Self {
        IcedMargin {
            top: p.t,
            right: p.r,
            bottom: p.b,
            left: p.l,
        }
    }
}

impl<'de, T> Deserialize<'de> for Insets<T>
where
    T: FromStr,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.trim().split_whitespace().collect();

        let parse = |v: &str| {
            v.parse().map_err(|_| {
                serde::de::Error::invalid_type(serde::de::Unexpected::Str(v), &"a number")
            })
        };

        if let [t, r, b, l] = parts[..] {
            return Ok(Insets {
                t: parse(t)?,
                b: parse(b)?,
                l: parse(l)?,
                r: parse(r)?,
            });
        }

        Err(serde::de::Error::invalid_length(
            4,
            &"expected 1, 2 or 4 arguments",
        ))
    }
}
