mod cli;
mod key;
mod keycode;
mod keymap;
mod svg_mod;

use std::path::PathBuf;

use key::Key;
use keymap::Keymap;
use svg_mod::Svg;
use cli::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();


    let keymap = Keymap::new(args.keymap);

    let mut keyboard = Key::generate_all_keys();

    keyboard = Key::add_layer(keymap.get_layer(args.layer), keyboard);

    let mut svg = Svg::new();
    svg.add_keyboard(keyboard);

    let image_path = args.image.unwrap_or(PathBuf::from(format!("layer{}.svg", args.layer)));

    svg.save(&image_path).expect("Error while saving svg.");
    println!("Image was saved to {:?}.", image_path);
}
