use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use svg::node;
use svg::node::element::{Group, Rectangle, Text};
use svg::Document;

const KEY_FILL_COL: &str = "#3f403f";
const KEY_BORDER_COLOR: &str = "#303030";
const KEY_BORDER_SIZE: u16 = 6;
const KEY_DIMENSIONS: u16 = 100;
const KEY_BORDER_RADIUS: u16 = 20;
const TEXT_COLOR: &str = "white";
const TEXT_SIZE: &str = "5em";

struct Key {
    label: String,
    x: u16,
    y: u16,
    mod_label: Option<String>,
}

impl Key {
    fn new(x: u16, y: u16, label: String) -> Self {
        Key {
            label,
            x,
            y,
            mod_label: None,
        }
    }
    fn add_mod_label(&mut self, label: String) {
        self.mod_label = Some(label);
    }
    fn svg(&self) -> Group {
        let key = Rectangle::new()
            .set("fill", KEY_FILL_COL)
            .set("stroke", KEY_BORDER_COLOR)
            .set("stroke-width", KEY_BORDER_SIZE)
            .set("x", self.x)
            .set("y", self.y)
            .set("rx", KEY_BORDER_RADIUS)
            .set("ry", KEY_BORDER_RADIUS)
            .set("width", KEY_DIMENSIONS)
            .set("height", KEY_DIMENSIONS);

        let letter = Text::new()
            .set("x", self.x + KEY_DIMENSIONS / 3)
            .set("y", self.y + KEY_DIMENSIONS / 2)
            .set("fill", TEXT_COLOR)
            .set("font-size", TEXT_SIZE)
            .add(node::Text::new(&self.label));

        Group::new().add(key).add(letter)
    }
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

#[allow(dead_code)]
#[derive(Deserialize)]
struct Keymap {
    version: u8,
    notes: String,
    documentation: String,
    keyboard: String,
    keymap: String,
    layout: String,
    layers: Vec<Vec<String>>,
    author: String,
}

fn generic_key_offset(num: u16) -> u16 {
    num * (KEY_DIMENSIONS + 10) + 20
}

fn main() {
    let mut keys = Vec::new();
    for y in 0..3 {
        for x in 0..10 {
            let x_offset = if x > 4 { 100 } else { 0 }; // This offset is for the split layout (second half)
            let y_offset = column_offset(x); // This offset is for the verical staggering of the keys
            keys.push(Key::new(
                generic_key_offset(x) + x_offset,
                generic_key_offset(y) + y_offset,
                "B".to_string(),
            ));
        }
    }
    // Thumb keys
    for x in 3..7 {
        let x_offset = if x > 4 { 100 } else { 0 }; // This offset is for the split layout (second half)
        let y_offset = column_offset(x) + 40; // This offset is for the verical staggering of the keys
        keys.push(Key::new(
            generic_key_offset(x) + x_offset,
            generic_key_offset(3) + y_offset,
            "B".to_string(),
        ))
    }

    let background = Rectangle::new()
        .set("fill", "grey")
        .set("height", 900u16)
        .set("width", 1300u16);
    let mut document = Document::new()
        .set("viewBox", (0u16, 0u16, 1250u16, 900u16))
        .add(background);

    for key in keys {
        document = document.add(key.svg());
    }

    // svg::save("image.svg", &document).unwrap();
    let data = fs::read_to_string("keymap.json").expect("Could not open file!");
    let v: Keymap = serde_json::from_str(&data).expect("Error while parsing json");
    println!("{}, {:?}", v.author, v.layers);
}
