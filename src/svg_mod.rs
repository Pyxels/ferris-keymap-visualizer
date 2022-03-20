use std::io;
use std::path::Path;

use svg::{node::element::Rectangle, Document};

use crate::key::Key;

pub struct Svg {
    document: Document,
}

impl Svg {
    pub fn new() -> Self {
        let background = Rectangle::new()
            .set("fill", "grey")
            .set("height", 900u16)
            .set("width", 1300u16);
        let document = Document::new()
            .set("viewBox", (0u16, 0u16, 1250u16, 900u16))
            .add(background);
        Svg { document }
    }
    pub fn add_keys(&mut self, keys: Vec<Key>) {
        let mut document = self.document.clone();
        for key in keys {
            document = document.add(key.svg());
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
