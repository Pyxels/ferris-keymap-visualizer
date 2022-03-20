use svg::node;
use svg::node::element::{Group, Rectangle, Text};

use crate::keycode::Keycode;

const KEY_FILL_COL: &str = "#3f403f";
const KEY_BORDER_COLOR: &str = "#303030";
const KEY_BORDER_SIZE: u16 = 6;
const KEY_DIMENSIONS: u16 = 100;
const KEY_BORDER_RADIUS: u16 = 20;
const TEXT_COLOR: &str = "white";
const TEXT_SIZE: &str = "3em";

pub struct Key {
    label: String,
    x: u16,
    y: u16,
    mod_label: Option<String>,
    text_size: String,
    text_y_offset: u16,
    text_x_offset: u16,
    background_color: String,
    border_color: String,
}

impl Key {
    fn new(x: u16, y: u16, label: String) -> Self {
        Key {
            label,
            x,
            y,
            mod_label: None,
            text_size: TEXT_SIZE.to_string(),
            text_y_offset: 0,
            text_x_offset: 0,
            background_color: KEY_FILL_COL.to_string(),
            border_color: KEY_BORDER_COLOR.to_string(),
        }
    }
    fn make_layer_key(&mut self) {
        self.background_color = "#ae1e1f".to_string();
        self.border_color = "#7a1919".to_string();
    }
    pub fn svg(&self) -> Group {
        let key = Rectangle::new()
            .set("fill", self.background_color.clone())
            .set("stroke", self.border_color.clone())
            .set("stroke-width", KEY_BORDER_SIZE)
            .set("x", self.x)
            .set("y", self.y)
            .set("rx", KEY_BORDER_RADIUS)
            .set("ry", KEY_BORDER_RADIUS)
            .set("width", KEY_DIMENSIONS)
            .set("height", KEY_DIMENSIONS);

        let letter = Text::new()
            .set("x", self.x + KEY_DIMENSIONS / 3 + self.text_x_offset)
            .set("y", self.y + KEY_DIMENSIONS / 2 + self.text_y_offset)
            .set("fill", TEXT_COLOR)
            .set("font-size", self.text_size.clone())
            .add(node::Text::new(&self.label));

        Group::new().add(key).add(letter)
    }
    fn add_keycode(&mut self, keycode: &Keycode) {
        if keycode.is_layer_toggle() {
            self.make_layer_key();
            self.label = "".to_string();
            return
        }
        self.label = keycode.to_string();
    }

    pub fn add_layer(keycodes: &Vec<Keycode>, mut keys: Vec<Self>) -> Vec<Self>{
        for (idx, el) in keycodes.iter().enumerate() {
            keys[idx].add_keycode(el);
        }
        keys
    }
    pub fn generate_all_keys(placeholder: String) -> Vec<Key> {
        let mut keys = Vec::new();
        for y in 0..3 {
            for x in 0..10 {
                let x_offset = if x > 4 { 100 } else { 0 }; // This offset is for the split layout (second half)
                let y_offset = Key::column_offset(x); // This offset is for the verical staggering of the keys
                keys.push(Key::new(
                    Key::generic_key_offset(x) + x_offset,
                    Key::generic_key_offset(y) + y_offset,
                    placeholder.to_string(),
                ));
            }
        }
        // Thumb keys
        for x in 3..7 {
            let x_offset = if x > 4 { 100 } else { 0 }; // This offset is for the split layout (second half)
            let y_offset = Key::column_offset(x) + 40; // This offset is for the verical staggering of the keys
            keys.push(Key::new(
                Key::generic_key_offset(x) + x_offset,
                Key::generic_key_offset(3) + y_offset,
                placeholder.to_string(),
            ))
        }
        keys
    }
    fn column_offset(column: u16) -> u16 {
        match column {
            0 => KEY_DIMENSIONS,
            1 => KEY_DIMENSIONS / 3,
            2 => 0,
            3 => KEY_DIMENSIONS / 3,
            4 => KEY_DIMENSIONS / 2,
            5 => KEY_DIMENSIONS / 2,
            6 => KEY_DIMENSIONS / 3,
            7 => 0,
            8 => KEY_DIMENSIONS / 3,
            9 => KEY_DIMENSIONS,
            _ => panic!("{column} Not a column index"),
        }
    }
    fn generic_key_offset(num: u16) -> u16 {
        num * (KEY_DIMENSIONS + 10) + 20
    }
}
