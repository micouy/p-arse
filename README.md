parsers

- [x] empty string (covered by string slices)
- [ ] terminals
  - [x] string slices (`"abc"`)
  - [x] chars (`'A'`)
  - [ ] char ranges (`'a'..'z'`)
  - [ ] regex, other `Pattern`s
- [x] non-terminals i.e. parser functions, including recursive functions
  ```
  fn a_string(tail: &str) -> Result<'_, ()> {
      ("a", a_string().opt())
          .ignore()
          .parse(tail)
  }
  ```
- [x] sequences (`("a", "b", "c")`)
- [x] prioritized choice (`"A".or("B")`)
- [x] zero or more repetitions (`"b".zore()`)
- [ ] not-predicate (`!"x"`)
- [ ] syntactic sugar
  - [x] any (`any()`)
  - [x] one or more (`"a".more()`)
  - [x] optionals (`"x".opt()`)
  - [ ] and-predicate (???)


other

- [ ] `.and_then()` for further processing that might fail
- [ ] clean error messages with parsing stack
- [ ] fix impl Parser for F and for &P (conflicting)
- [ ] make each parser return the whole string slice it has captured (its children's captures concatenated)


reference

1. https://bford.info/pub/lang/peg.pdf
