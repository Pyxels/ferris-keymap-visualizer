use std::fs;
use std::path::Path;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Keymap {
    version: u8,
    notes: String,
    documentation: String,
    keyboard: String,
    keymap: String,
    layout: String,
    layers: Vec<Vec<String>>,
    author: String,
}
impl Keymap {
    pub fn new<T>(path: T) -> Self
    where
        T: AsRef<Path>,
    {
        // TODO: work with result
        let keymap_raw = fs::read_to_string(path).expect("Could not open file!");
        serde_json::from_str(&keymap_raw).expect("Error while parsing json")
    }
    pub fn get_layer(&self, idx: usize) -> &Vec<String> {
        &self.layers[idx]
    }
}
