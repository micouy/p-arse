use std::iter::FromIterator;

use p_arse::{CharExt, Parser};

fn main() {
    let print_letter = |letter| {
        println!("Rule:     letter");

        letter
    };
    let print_digit = |digit| {
        println!("Rule:     digit");

        digit
    };
    let print_ident = |cs| {
        println!("Rule:     ident");
    };
    let print_text = |text: &str| {
        println!("Text:     {}", text);
    };

    let alpha = ('a'.to('z'))
        .or('A'.to('Z'))
        .map(print_letter)
        .maps(print_text);
    let digit = '0'.to('9').map(print_digit).maps(print_text);
    let ident = alpha.or(digit).more().map(print_ident).maps(print_text);
    let ident_list = (digit.not_ahead(), ident, (" ", ident).more());

    let _ = ident_list.p_arse("a1 b2").unwrap();
}
