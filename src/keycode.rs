use std::fmt;

pub struct Keycode {
    keystring: String,
    modifier: Option<Modifier>,
    current_layer_toggle: bool,
}

enum Modifier {
    Shift,
    Alt,
    Ctrl,
    Layer(u8),
}

impl Keycode {
    pub fn new(key: String) -> Self {
        Self::parse_keycode(&key)
    }

    fn parse_keycode(keycode: &str) -> Keycode {
        // TODO Result
        let mut keystring;
        let mut modifier = None;

        if keycode.contains("(") {
            let mut split = keycode.split("(");
            let first = split.next().unwrap();

            if first == "LT" {
                let mut layer_info = split.next().unwrap().split(",");
                let layer_number: u8 = layer_info.next().unwrap().parse().unwrap();
                modifier = Some(Modifier::Layer(layer_number));
                keystring = layer_info.next().unwrap().to_string();
                keystring = Self::match_keystring(keystring);
            } else if ["LCTL", "LCA", "DF"].contains(&first) {
                let inside_key = split.next().unwrap();
                keystring = match first {
                    "LCTL" => "⌃",
                    "LCA" => "⌃⌥",
                    "DF" => "",
                    _ => panic!("How did you even get here?"),
                }
                .to_string();
                keystring.push_str(&Self::match_keystring(inside_key.to_string()));
            } else {
                modifier = Some(match first {
                    "LSFT_T" => Modifier::Shift,
                    "LCTL_T" => Modifier::Ctrl,
                    "LCTL" => Modifier::Ctrl,
                    "LALT_T" => Modifier::Alt,
                    "RALT_T" => Modifier::Alt,
                    "RCTL_T" => Modifier::Ctrl,
                    x => panic!("Not a known modifier! {x}"),
                });
                keystring = split.next().unwrap().to_string();
                keystring = Self::match_keystring(keystring);
            }
        } else {
            keystring = keycode.to_string();
            keystring = Self::match_keystring(keystring);
        }

        let current_layer_toggle = if keystring == "NO" { true } else { false };
        Keycode {
            keystring,
            modifier,
            current_layer_toggle,
        }
    }
    fn match_keystring(keystring: String) -> String {
        let mut keystring = keystring.replace(")", "");
        keystring = keystring.splitn(2, "_").last().unwrap().to_string();

        match keystring.as_str() {
            "SPC" => "⎵",
            "BSPC" => "",
            "TAB" => "",
            "LGUI" => "",
            "HASH" => "# '",
            "MINS" => "- _",
            "COMM" => ", ;",
            "DOT" => ". :",
            "BTN1" => " 1",
            "BTN2" => " 2",
            "WH_U" => "",
            "WH_D" => "",
            "WH_L" => "",
            "WH_R" => "",
            "MS_L" => "",
            "MS_D" => "",
            "MS_U" => "",
            "MS_R" => "",
            "LEFT" => "",
            "DOWN" => "",
            "UP" => "",
            "RGHT" => "",
            "END" => "E",
            "HOME" => "",
            "PGUP" => "P",
            "PGDN" => "P",
            "LALT" => "⌥",
            "LSFT" => "⇧",
            "CIRC" => "^",
            "ASTR" => "*",
            "AMPR" => "&amp;",
            "UNDS" => "_",
            "PIPE" => "|",
            "TILD" => "~",
            "SLSH" => "/",
            "BSLS" => "\\",
            "GRV" => "`",
            "DQUO" => "\"",
            "QUOT" => "'",
            "DLR" => "$",
            "TRNS" => "",
            letter => letter,
        }
        .to_string()
    }
    pub fn is_layer_toggle(&self) -> bool {
        self.current_layer_toggle
    }
}

impl fmt::Display for Keycode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.keystring)
    }
}
