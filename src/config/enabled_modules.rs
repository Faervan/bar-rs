use configparser::ini::Ini;


#[derive(Debug)]
pub struct EnabledModules {
    pub left: Vec<String>,
    pub center: Vec<String>,
    pub right: Vec<String>,
}

impl Default for EnabledModules {
    fn default() -> Self {
        let vec = |list: &[&str]|
            list.into_iter()
                .map(|i| i.to_string())
                .collect();

        Self {
            left: vec(&[
                "hyprland.workspaces",
                "hyprland.window"
            ]),
            center: vec(&["time"]),
            right: vec(&[
                "media",
                "volume",
                "cpu",
                "memory"
            ]),
        }
    }
}

impl From<&Ini> for EnabledModules {
    fn from(ini: &Ini) -> Self {
        let get = move |field: &str|
            ini.get("modules", field)
                .map(|value| value.split(',')
                    .map(|v| v.trim().to_string())
                    .collect());

        let default = Self::default();

        Self {
            left: get("left")
                .unwrap_or(default.left),
            center: get("center")
                .unwrap_or(default.center),
            right: get("right")
                .unwrap_or(default.right),
        }
    }
}

impl EnabledModules {
    pub fn get_all(&self) -> impl Iterator<Item = &String> {
        self.left.iter()
            .chain(self.center.iter())
            .chain(self.right.iter())
    }

    pub fn write_to_ini(&self, ini: &mut Ini) {
        ini.set("modules", "left", Some(self.left.join(", ")));
        ini.set("modules", "center", Some(self.center.join(", ")));
        ini.set("modules", "right", Some(self.right.join(", ")));
    }
}
