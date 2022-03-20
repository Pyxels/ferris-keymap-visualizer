
mod key;
mod keymap;
mod svg_mod;

use key::Key;
use keymap::Keymap;
use svg_mod::Svg;


fn main() {
    
    let keymap = Keymap::new("keymap.json".to_string());

    let mut keys = Key::generate_all_keys("placeholder".to_string());

    for (idx, el) in keymap.get_layer(0).iter().enumerate() {
        keys[idx].add_keycode(el.clone());
    }

    let mut svg = Svg::new();
    svg.add_keys(keys);

    svg.save("image.svg").expect("Error while saving svg.");
}
