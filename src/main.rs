use svg::node;
use svg::node::element::{Group, Rectangle, Text};
use svg::Document;

const KEY_FILL_COL: &str = "#3f403f";
const KEY_BORDER_COLOR: &str = "#303030";
const KEY_BORDER_SIZE: i16 = 6;
const KEY_DIMENSIONS: i16 = 100;
const KEY_BORDER_RADIUS: i16 = 20;
const TEXT_COLOR: &str = "white";
const TEXT_SIZE: &str = "5em";

struct Key {
    label: String,
    x: i16,
    y: i16,
    mod_label: Option<String>,
}

impl Key {
    fn new(x: i16, y: i16, label: String) -> Self {
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

fn main() {
    let key = Key::new(20, 20, "A".to_string());

    let group = key.svg();

    let document = Document::new().set("viewBox", (0, 0, 200, 200)).add(group);

    svg::save("image.svg", &document).unwrap();
}
