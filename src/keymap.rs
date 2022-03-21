use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::keycode::Keycode;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Keymap {
    version: u8,
    notes: String,
    documentation: String,
    keyboard: String,
    #[serde(default)]
    host_language: String,
    keymap: String,
    layout: String,
    layers: Vec<Vec<String>>,
    author: String,
    #[serde(skip)]
    keycodes: Vec<Vec<Keycode>>,
}
impl Keymap {
    pub fn new<T>(path: T) -> Self
    where
        T: AsRef<Path>,
    {
        // TODO: work with result
        let keymap_raw = fs::read_to_string(path).expect("Could not open file!");
        let mut keymap: Self = serde_json::from_str(&keymap_raw).expect("Error while parsing json");
        keymap.parse_keys();
        keymap
    }
    pub fn get_layer(&self, idx: usize) -> &Vec<Keycode> {
        if idx >= self.keycodes.len() {
            eprintln!(
                "Index {idx} is out of bounds. There are only {} layers. Exiting...",
                self.keycodes.len()
            );
            std::process::exit(1);
        }
        &self.keycodes[idx]
    }
    fn parse_keys(&mut self) {
        // TODO: Result and custom error

        for layer in &self.layers {
            let mut layer_keys = Vec::new();
            for key in layer {
                layer_keys.push(Keycode::new(key.clone()));
            }
            self.keycodes.push(layer_keys);
        }
    }
}
