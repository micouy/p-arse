use p_arse::{Parser, Result, any};

#[test]
fn test_literals() {
    let just_a = 'a'; // "a"
    assert!(just_a.p_arse("a").is_ok());
    assert!(just_a.p_arse("b").is_err());

    let abc = "abc"; // "abc"
    assert!(abc.p_arse("abc").is_ok());
    assert!(abc.p_arse("def").is_err());
}

#[test]
fn test_sequences() {
    let a = ("a",); // "a"
    assert!(a.p_arse("a").is_ok());
    assert!(a.p_arse("x").is_err());

    let ab = ("a", "b"); // "a" "b"
    assert!(ab.p_arse("ab").is_ok());
    assert!(ab.p_arse("xx").is_err());

    let abc = ("a", "b", "c"); // "a" "b" "c"
    assert!(abc.p_arse("abc").is_ok());
    assert!(abc.p_arse("xxx").is_err());
}

#[test]
fn test_prioritized_choice() {
    let a_or_b_or_c = "a".or("b").or("c"); // "a" / "b" / "c"
    assert!(a_or_b_or_c.p_arse("axxx").is_ok());
    assert!(a_or_b_or_c.p_arse("bxxx").is_ok());
    assert!(a_or_b_or_c.p_arse("cxxx").is_ok());
}

#[test]
fn test_mapping() {
    let parse_digit = |d: char| d.to_digit(10).unwrap();
    let digit = any().map(parse_digit);
    assert_eq!(digit.p_arse("1").unwrap().0, 1);
}

#[test]
fn test_recursive_parsers() {
    // A = "a" A?
    fn a_string<'a>(tail: &'a str) -> Result<'a, ()> {
        ("a", a_string.opt()).ignore().p_arse(tail)
    }

    assert!(a_string.p_arse("").is_err());
    assert!(a_string.p_arse("a").is_ok());
    assert!(a_string.p_arse("aa").is_ok());
    assert!(a_string.p_arse("aaa").is_ok());
}

#[test]
fn test_repetition() {
    let bees = "b".zore(); // "b"*
    assert!(bees.p_arse("").is_ok());
    assert!(bees.p_arse("b").is_ok());
    assert!(bees.p_arse("bb").is_ok());
    assert!(bees.p_arse("bbb").is_ok());

    let scream = "a".more(); // "a"+
    assert!(scream.p_arse("").is_err());
    assert!(scream.p_arse("a").is_ok());
    assert!(scream.p_arse("aa").is_ok());
    assert!(scream.p_arse("aaa").is_ok());
}

#[test]
fn test_lookaheads() {
    let a_ahead = "a".ahead(); // &"a"
    assert!(a_ahead.p_arse("aaa").is_ok());
    assert!(a_ahead.p_arse("bbb").is_err());

    let a_not_ahead = "a".not_ahead(); // !"a"
    assert!(a_not_ahead.p_arse("bbb").is_ok());
    assert!(a_not_ahead.p_arse("aaa").is_err());
}
