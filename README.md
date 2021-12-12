# `( ㅅ )`

> `p-arse` — the inelegant parser


**parsers**

- [x] empty string (covered by string slices)
- [ ] terminals
  - [x] string slices (`"abc"`)
  - [x] chars (`'A'`)
  - [x] char ranges (`'a'.to('z')`)
  - [ ] regex, other `Pattern`s
- [x] non-terminals i.e. parser functions, including recursive functions
  ```
  fn a_string(tail: &str) -> Result<'_, ()> {
      ('a', a_string.opt())
          .ignore()
          .parse(tail)
  }
  ```
- [x] sequences (`(a, b, c)`)
- [x] prioritized choice (`a.or(b)`)
- [x] zero or more repetitions (`a.zore()`)
- [x] not-predicate (`a.not_ahead()`, looking for a more concise name)
- [x] end of input (`eoi()`)
- [x] syntactic sugar
  - [x] any (`any()`)
  - [x] one or more repetitions (`a.more()`)
  - [x] optionals (`a.opt()`)
  - [x] and-predicate (`a.ahead()`, looking for a more concise name)


**todo**

- [ ] add `.and_then()` for further processing that might fail
- [ ] clean error messages with parsing stack
- [ ] fix impl Parser for F and for &P (conflicting)
- [ ] make each parser return the whole string slice it has captured (its children's captures concatenated)
- [ ] add docs
- [ ] add a method that would parse and return only the parsed value (?)


**reference**

1. https://bford.info/pub/lang/peg.pdf
