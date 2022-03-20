use std::fmt;

pub struct Keycode {
    key: String,
    layer: Option<u8>,
    modifier: Option<String>,
}

impl Keycode {
    pub fn new(key: String) -> Self {
        Keycode {
            key,
            layer: None,
            modifier: None,
        }
    }

    pub fn parse_keycode(keycode: &str) -> Keycode {
        let mut keycode = keycode.replace(")", "");
        keycode = keycode.split("_").last().unwrap().to_string();

        Keycode::new(match keycode.as_str() {
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
        }.to_string())
    }
}

impl fmt::Display for Keycode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.key)
    }
}
