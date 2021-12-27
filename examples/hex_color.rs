// use std::iter::FromIterator;

use p_arse::{CharExt, Parser, TupleExt};

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let parse_hex_dd = |s: &str| u8::from_str_radix(s, 16).unwrap();
    let construct_color = |(r, g, b)| Color { r, g, b };

    let hex_d = ('0'.to('9')).or('a'.to('f'));
    let hex_dd = (hex_d, hex_d).maps(parse_hex_dd);
    let color = ("#", hex_dd, hex_dd, hex_dd).r0().map(construct_color);

    let (color, _tail) = color.p_arse("#defec8").unwrap();

    dbg!(&color);

    assert_eq!(color.r, 0xde);
    assert_eq!(color.g, 0xfe);
    assert_eq!(color.b, 0xc8);
}
