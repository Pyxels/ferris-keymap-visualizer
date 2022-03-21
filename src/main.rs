mod key;
mod keycode;
mod keymap;
mod svg_mod;

use key::Key;
use keymap::Keymap;
use svg_mod::Svg;

fn main() {
    let keymap = Keymap::new("keymap.json".to_string());

    let mut keyboard = Key::generate_all_keys();

    keyboard = Key::add_layer(keymap.get_layer(1), keyboard);

    let mut svg = Svg::new();
    svg.add_keyboard(keyboard);

    svg.save("image.svg").expect("Error while saving svg.");
}
