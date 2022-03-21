mod cli;
mod key;
mod keycode;
mod keymap;
mod svg_mod;

use std::path::PathBuf;

use clap::Parser;
use cli::Args;
use key::Key;
use keymap::Keymap;
use svg_mod::Svg;

fn main() {
    let args = Args::parse();

    let keymap = Keymap::new(args.keymap);
    let image_path;
    let mut svg;

    if let Some(layer) = args.layer {
        let mut keyboard = Key::generate_all_keys();
        keyboard = Key::add_layer(keymap.get_layer(layer), keyboard);

        svg = Svg::new(1);
        svg.add_keyboard(keyboard, 0);

        image_path = args
            .image
            .unwrap_or(PathBuf::from(format!("layer{}.svg", layer)));
    } else {
        let layer_count = keymap.layer_count();

        svg = Svg::new(layer_count);

        for layer in 0..layer_count {
            let mut keyboard = Key::generate_all_keys();
            keyboard = Key::add_layer(keymap.get_layer(layer), keyboard);
            svg.add_keyboard(keyboard, layer);
        }

        image_path = args
            .image
            .unwrap_or(PathBuf::from(format!("layer0-{layer_count}.svg")));
    }
    svg.save(&image_path).expect("Error while saving svg.");
    println!("Image was saved to {:?}.", image_path);
}
