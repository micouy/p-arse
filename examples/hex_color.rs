use std::iter::FromIterator;

use p_arse::traits::*;

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let parse_hex_dd = |(c1, c2): (char, char)| {
        u8::from_str_radix(&String::from_iter([c1, c2]), 16).unwrap()
    };
    let construct_color = |(_, r, g, b)| Color { r, g, b };

    let hex_d = ('0'.to('9')).or('a'.to('f'));
    let hex_dd = (hex_d, hex_d).map(parse_hex_dd);
    let color = ("#", hex_dd, hex_dd, hex_dd).map(construct_color);

    let (color, _tail): (Color, _) = color.p_arse("#defec8").unwrap();

    dbg!(&color);

    assert_eq!(color.r, 0xde);
    assert_eq!(color.g, 0xfe);
    assert_eq!(color.b, 0xc8);
}
