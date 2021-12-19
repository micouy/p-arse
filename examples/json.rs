use std::{collections::HashMap, iter::once};

use p_arse::{
    any,
    function::{Rec, RecursiveFunction},
    rec,
    CharExt,
    Parser,
};

// pub enum JsonValue {
// Null,
// Bool(bool),
// Str(String),
// Num(f64),
// Array(Vec<JsonValue>),
// Object(HashMap<String, JsonValue>),
// }
// use JsonValue::*;

fn main() {
    let non_zero = ('1'.to('9'), '0'.to('9').zore()).ignore();
    let zero = '0'.ignore();
    let whole = non_zero.or(zero);
    let fractional = ('.', '0'.to('9').more());
    let exponential = ('e'.or('E'), '+'.or('-').opt(), whole);
    let number = (whole, fractional.opt(), exponential.opt()).ignore();

    let boolean = ("true".map(|_| true))
        .or("false".map(|_| false))
        // .map(|value| Ok(Bool(value)))
        .ignore();

    let collect_string =
        |(_, cs, _): (char, Vec<char>, char)| cs.iter().collect::<String>();

    let special = '\\'
        .or('/')
        .or('"')
        .or('b'.map(|_| '\x08'))
        .or('n'.map(|_| '\n'))
        .or('f'.map(|_| '\x0C'))
        .or('r'.map(|_| '\r'))
        .or('t'.map(|_| '\t'));
    let escape_sequence = ('\\', special).map(|(_, special)| special);
    let not_escape_or_ending =
        ('"'.or('\\').not_ahead(), any()).map(|(_, c)| c);
    let string = ('"', not_escape_or_ending.or(escape_sequence).zore(), '"')
        .map(collect_string)
        .ignore();

    // After resolving #86921:
    // let array = |value| ...;
    fn array(
        value: RecursiveFunction<'_, ()>,
    ) -> impl Parser<Output = ()> + '_ {
        ('[', (value, (',', value).zore()).opt(), ']').ignore()
    }

    // After resolving #86921:
    // let object = |value| ...;
    fn object<'a>(
        string: impl Parser<Output = ()> + 'a,
        value: RecursiveFunction<'a, ()>,
    ) -> impl Parser<Output = ()> + 'a {
        (
            '{',
            ((string, ':', value), (',', (string, ':', value)).zore()).opt(),
            '}',
        )
            .ignore()
    }

    let value: &dyn Rec<_> = &|tail, value| {
        number
            .or(string)
            .or(boolean)
            .or(array(value))
            .or(object(string, value))
            .p_arse(tail)
    };
    let value = rec(value);

    let input = r#"
	{
        "Image": {
            "Width":  800,
            "Height": 600,
            "Title":  "View from 15th Floor",
            "Thumbnail": {
                "Url":    "http://www.example.com/image/481989943",
                "Height": 125,
                "Width":  100
            },
            "Animated" : false,
            "IDs": [116, 943, 234, 38793]
        }
    }"#;

    dbg!("numbers");
    value.p_arse("123").unwrap();
    value.p_arse("123.123").unwrap();
    value.p_arse("123.123e-123").unwrap();
    value.p_arse("123e-123").unwrap();

    dbg!("bools");
    value.p_arse("true").unwrap();
    value.p_arse("false").unwrap();

    dbg!("strings");
    value.p_arse(r#""abc""#).unwrap();
    value.p_arse(r#""sdf\ns\\d\"f""#).unwrap();

    dbg!("arrays");
    value.p_arse(r#"[]"#).unwrap();
    value.p_arse(r#"[1,2,3]"#).unwrap();
    value.p_arse(r#"[1,"xd",3]"#).unwrap();
    value.p_arse(r#"[1,"xd",[1,true,3]]"#).unwrap();

    dbg!("object");
    value.p_arse(r#"{}"#).unwrap();
    value.p_arse(r#"{"xd":3,"xx":[1,true,3]}"#).unwrap();
}
