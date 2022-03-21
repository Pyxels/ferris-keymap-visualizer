mod key;
mod keymap;
mod svg_mod;
mod keycode;

use key::Key;
use keymap::Keymap;
use svg_mod::Svg;

fn main() {
    let keymap = Keymap::new("keymap.json".to_string());

    let mut keyboard = Key::generate_all_keys("placeholder".to_string());

    keyboard = Key::add_layer(keymap.get_layer(1), keyboard);

    let mut svg = Svg::new();
    svg.add_keyboard(keyboard);

    svg.save("image.svg").expect("Error while saving svg.");
}
