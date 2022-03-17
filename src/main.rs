use svg::node;
use svg::node::element::{Rectangle, Text};
use svg::Document;

fn main() {

    let key = Rectangle::new()
        .set("fill", "#3f403f")
        .set("stroke", "#303030")
        .set("stroke-width", 6_i16)
        .set("x", 20_i16)
        .set("y", 10_i16)
        .set("rx", 20_i16)
        .set("ry", 20_i16)
        .set("width", 100_i16)
        .set("height", 100_i16);

    let letter = Text::new()
        .set("x", 50)
        .set("y", 80)
        .set("fill", "white")
        .set("font-size", "5em")
        .add(node::Text::new("T"));


    let document = Document::new().set("viewBox", (0, 0, 200, 200)).add(key).add(letter);

    svg::save("image.svg", &document).unwrap();
}
