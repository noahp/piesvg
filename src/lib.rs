// Oh Hi! this is a real dumb library that can produce a completely
// non-customizable svg pie chart. See something like pygal for a non terrible
// version of this, ok see you later.

extern crate svg;

use svg::node::element::Circle;
use svg::Document;

// Slice data struct
#[derive(Debug)]
pub struct Pieslice {
    // category name
    name: String,
    // slice size
    value: f32,
}

#[cfg(test)]
mod tests {
    use make_svg;
    use Pieslice;
    #[test]
    fn it_works() {
        let slicies: [Pieslice; 2] = [
            Pieslice {
                name: "one".to_string(),
                value: 1.2,
            },
            Pieslice {
                name: "two".to_string(),
                value: 2.3,
            },
        ];
        make_svg("test.svg", &slicies);
    }
}

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

pub fn make_svg(outfilename: &str, slices: &[Pieslice]) {
    println!("slicies: {:?}", slices);

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

    svg::save(outfilename, &document).unwrap()
}
