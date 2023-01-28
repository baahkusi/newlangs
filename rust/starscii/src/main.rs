use std::env;
use starscii::fonts::{StarsciiConfig, Starscii, boxfont::BoxFont};
fn main() {
    let args: Vec<String> = env::args().collect();
    let text = args.get(1).unwrap();
    let filler = args.get(2).unwrap_or(&String::from("*")).trim().chars().next().expect("valid single characters expected");
    let font_size: u8 = args.get(3).unwrap_or(&String::from("5")).trim().parse().expect("must be a positive number");
    let config = StarsciiConfig {
        font_size,
        font_type: Box::new(BoxFont{}),
        filler
    };
    let starscii = Starscii::new(config);

    println!("{}", starscii.draw(text));
}
