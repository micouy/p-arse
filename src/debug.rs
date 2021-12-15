use p_arse::{any, rec, CharExt, Parser, Result};

fn main() {
    let a_string = rec(&|tail: &str, a_string| -> Result<()> {
        dbg!(tail);
        ("a", a_string.opt()).ignore().p_arse(tail)
    });

    assert!(a_string.p_arse("aa").is_ok());
}
