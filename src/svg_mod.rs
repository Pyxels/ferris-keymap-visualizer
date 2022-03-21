use std::io;
use std::path::Path;

use svg::{node::element::Rectangle, Document};

use crate::key::Key;

pub struct Svg {
    document: Document,
}

impl Svg {
    pub fn new(layer_count: usize) -> Self {
        let height = div_round_up(layer_count as u16, 2) * 600;
        let width = if layer_count > 1 { 2500 } else { 1250 };
        let background = Rectangle::new()
            .set("fill", "grey")
            .set("height", height)
            .set("width", width);
        let document = Document::new()
            .set("viewBox", (0u16, 0u16, width, height))
            .add(background);
        Svg { document }
    }
    pub fn add_keyboard(&mut self, keys: Vec<Key>, layer_index: usize) {
        let x_offset = (layer_index % 2) as u16 * 1250;
        let y_offset = layer_index as u16 / 2 * 600;
        let mut document = self.document.clone();
        for key in keys {
            document = document.add(key.svg(x_offset, y_offset));
        }
        self.document = document;
    }
    pub fn save<T>(&self, path: T) -> io::Result<()>
    where
        T: AsRef<Path>,
    {
        svg::save(path, &self.document)
    }
}
fn div_round_up(a: u16, b: u16) -> u16 {
    (a + (b - 1)) / b
}
