# `p-( ㅅ )`

> `p-arse` — the inelegant parser


## description

`p-arse` is a [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) parser library focused on readability and type safety • it follows the syntax from the [original paper](https://bford.info/pub/lang/peg.pdf) as closely as possible • the parsers are point-free (they're (mostly) variables, not functions), as opposed to [`nom`][nom]'s parsers which are functions or compositions of functions • this encourages the user to bind and name many intermediate parsers • it is similar to [`pest`][pest] in this regard


## example

```rust
let parse_hex_dd = |s: &str| {
    u8::from_str_radix(s, 16).unwrap()
};
let construct_color = |(r, g, b)| Color { r, g, b };

let hex_d = ('0'.to('9')).or('a'.to('f'));
let hex_dd = (hex_d, hex_d).maps(parse_hex_dd);
let color = ("#", hex_dd, hex_dd, hex_dd).r0().map(construct_color);

let (color, _tail) = color.p_arse("#defec8").unwrap();
```

check out [other examples](examples/) • i've replicated examples from other parser libaries, i.e. [`nom`'s hex color](https://github.com/Geal/nom#example) ([mine](examples/hex_color.rs)) and [`pest`'s ident list](https://github.com/pest-parser/pest#example) ([mine](examples/ident.rs))


## todo

- [ ] add docs
- [ ] add verbose error messages
- [ ] allow access to the string slice captured by the parser (its children's captures concatenated) (kinda works, except where `.rn()` is used)


## reference

1. [https://bford.info/pub/lang/peg.pdf]

[nom]: https://github.com/Geal/nom 
[pest]: https://github.com/pest-parser/pest
