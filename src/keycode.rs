use std::fmt;

pub struct Keycode {
    key: String,
    layer: Option<u8>,
    modifier: Option<String>,
    current_layer_toggle: bool,
}

impl Keycode {
    pub fn new(key: String) -> Self {
        let key = Self::parse_keycode(&key);

        let current_layer_toggle = if key.to_string() == "NO" { true } else { false };
        Keycode {
            key,
            layer: None,
            modifier: None,
            current_layer_toggle,
        }
    }

    fn parse_keycode(keycode: &str) -> String {
        let mut keycode = keycode.replace(")", "");
        keycode = keycode.split("_").last().unwrap().to_string();

        match keycode.as_str() {
            "SPC" => "",
            "BSPC" => "",
            "TAB" => "",
            "LGUI" => "",
            "HASH" => "# '",
            "MINS" => "- _",
            "COMM" => ", ;",
            "DOT" => ". :",
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
        write!(f, "{}", self.key)
    }
}
