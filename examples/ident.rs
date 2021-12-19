use std::iter::FromIterator;

use p_arse::{CharExt, Parser};

fn main() {
    let print_letter = |letter| {
        println!("Letter:   {}", letter);
        letter
    };
    let print_digit = |digit| {
        println!("Digit:    {}", digit);
        digit
    };
    let print_ident = |cs| {
        println!("Ident:    {}", String::from_iter(cs));
    };

    let alpha = ('a'.to('z')).or('A'.to('Z')).map(print_letter);
    let digit = '0'.to('9').map(print_digit);
    let ident = alpha.or(digit).more().map(print_ident);
    let ident_list = (digit.not_ahead(), ident, (" ", ident).more());

    let _ = ident_list.p_arse("a1 b2").unwrap();
}
