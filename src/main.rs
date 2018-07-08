// Cli utility that produces an svg from input args.

extern crate piesvg;

fn main() {
    println!("Making an svg pie chart!");

    piesvg::make_svg("image.svg", &[]);
}
