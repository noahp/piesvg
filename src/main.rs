extern crate piesvg;
extern crate svg;

use svg::Document;
use svg::node::element::Circle;

fn get_segment(stroke: &str, dasharray: (usize, usize), dashoffset: usize) -> Circle {
    Circle::new()
        .set("class", "donut-segment")
        .set("cx", 21)
        .set("cy", 21)
        .set("r", 10)
        .set("fill", "transparent")
        .set("stroke", stroke)
        .set("stroke-width", 20)
        .set("stroke-dasharray", dasharray)
        .set("stroke-dashoffset", dashoffset)
}

fn main() {
    println!("Making an svg pie chart!");

    let donut_hole = Circle::new()
        .set("class", "donut-hole")
        .set("cx", 21)
        .set("cy", 21)
        .set("r", 10)
        .set("fill", "#fff");

    let donut_ring = Circle::new()
        .set("class", "donut-ring")
        .set("cx", 21)
        .set("cy", 21)
        .set("r", 10)
        .set("fill", "transparent")
        .set("stroke", "#d2d3d4")
        .set("stroke-width", 20);

    let document = Document::new()
        .set("xmlns:xlink", "http://www.w3.org/1999/xlink")
        .set("width", "100%")
        .set("height", "100%")
        .set("viewBox", (0, 0, 42, 42))
        .set("class", "donut")
        .add(donut_hole)
        .add(donut_ring)
        .add(get_segment("#ce4b99", (40, 60), 25))
        .add(get_segment("#b1c94e", (20, 80), 85))
        .add(get_segment("#377bbc", (30, 70), 65));

    svg::save("image.svg", &document).unwrap()
}
