// Based on [https://www.crockford.com/mckeeman.html].

use std::{collections::HashMap, iter::once};

use p_arse::{
    any,
    function::{Rec, RecursiveFunction},
    rec,
    CharExt,
    Parser,
};

#[derive(Debug)]
pub enum Json {
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Array),
    Object(Object),
}

type Array = Vec<Json>;
type Object = HashMap<String, Json>;

use Json::*;

#[derive(Debug)]
struct Error;

type Result<T> = std::result::Result<T, Error>;

fn main() {
    let null = "null";

    let boolean = ("true".map(|_| true)).or("false".map(|_| false));

    let number = {
        let parse_number = |s: &str| s.parse::<f64>().map_err(|_| Error);

        let non_zero = ('1'.to('9'), '0'.to('9').zore()).ignore();
        let zero = '0'.ignore();
        let whole = non_zero.or(zero);
        let fractional = ('.', '0'.to('9').more());
        let exponential = ('e'.or('E'), '+'.or('-').opt(), whole);
        let number =
            (whole, fractional.opt(), exponential.opt()).maps(parse_number);

        number
    };

    let string = {
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
        let string = (
            '"',
            not_escape_or_ending
                .or(escape_sequence)
                .zore()
                .maps(|s| s.to_string()),
            '"',
        )
            .map(|(_, content, _)| content);

        string
    };

    let ws = ' '.or('\n').or('\t').zore().ignore();

    fn array<'a>(
        value: RecursiveFunction<'a, Result<Json>>,
        ws: impl Parser<Output = ()> + 'a,
    ) -> impl Parser<Output = Result<Array>> + 'a {
        let collect_elements =
            |(first, rest)| once(first).chain(rest).collect::<Result<Array>>();

        let element = (ws, value, ws).map(|(_, value, _)| value);
        let rest = (',', element).map(|(_, element)| element).zore();
        let elements = (element, rest).map(collect_elements);
        let empty_array = ('[', ws, ']').map(|_| Ok(Array::new()));
        let non_empty_array =
            ('[', elements, ']').map(|(_, elements, _)| elements);
        let array = empty_array.or(non_empty_array);

        array
    }

    fn object<'a>(
        string: impl Parser<Output = String> + 'a,
        value: RecursiveFunction<'a, Result<Json>>,
        ws: impl Parser<Output = ()> + 'a,
    ) -> impl Parser<Output = Result<Object>> + 'a {
        let collect_members =
            |(first, rest)| once(first).chain(rest).collect::<Result<Object>>();

        let element = (ws, value, ws).map(|(_, value, _)| value);
        let member = (ws, string, ws, ':', element)
            .map(|(_, string, _, _, element)| Ok((string, element?)));
        let rest = (',', member).map(|(_, member)| member).zore();
        let members = (member, rest).map(collect_members);
        let empty_object = ('{', ws, '}').map(|_| Ok(Object::new()));
        let non_empty_object =
            ('{', members, '}').map(|(_, members, _)| members);
        let object = empty_object.or(non_empty_object);

        object
    }

    let value: &dyn Rec<_> = &|tail, value| {
        object(string, value, ws)
            .map(|object| object.map(Object))
            .or(array(value, ws).map(|array| array.map(Array)))
            .or(number.map(|number| number.map(Num)))
            .or(string.map(|string| Ok(Str(string))))
            .or(boolean.map(|boolean| Ok(Bool(boolean))))
            .or(null.map(|_| Ok(Null)))
            .p_arse(tail)
    };
    let value = rec(value);

    let json = (ws, value, ws).map(|(_, value, _)| value);

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

    let (json, _) = json.p_arse(input).unwrap();
    dbg!(json.unwrap());
}
