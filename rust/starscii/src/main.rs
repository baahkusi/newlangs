use starscii::fonts::{StarsciiConfig, Starscii, boxfont::BoxFont};
fn main() {
    let config = StarsciiConfig {
        font_size: 5,
        font_type: Box::new(BoxFont{}),
        filler: '*'
    };
    let starscii = Starscii::new(config);

    println!("{}", starscii.draw("c C"));
}
