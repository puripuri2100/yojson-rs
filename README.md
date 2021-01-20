# yojson-rs: JSON library for Rust

[![Build Status][ci-badge]][ci]
[![source badge][source-badge]][source]
[![license badge][license-badge]][license]

[ci]: https://github.com/puripuri2100/yojson-rs/actions?query=workflow%3ACI
[ci-badge]: https://github.com/puripuri2100/yojson-rs/workflows/CI/badge.svg?branch=master
[source]: https://github.com/puripuri2100/yojson-rs
[source-badge]: https://img.shields.io/badge/source-github-blue
[license]: https://github.com/puripuri2100/yojson-rs/blob/master/LICENSE
[license-badge]: https://img.shields.io/badge/license-MIT-blue


This library parses [JSON data (yojson format)](https://mjambon.github.io/mjambon2016/yojson.html) into a nested Rust tree data structure.

# Yojson values

A value in Yojson is represented with the `Value` enum in this crate:

```rust
pub enum Value {
  Null,
  Bool(bool),
  Integer(i64),
  Float(f64),
  String(String),
  Assoc(Assoc),
  Array(Array),
  Tuple(Vec<Value>),
  Variant(Variant),
}
```

The Yojson format is an extension of the JSON format. See ["Yojson format document"](https://mjambon.github.io/mjambon2016/yojson.html) for more information.

- Tuples: like JSON arrays but within parentheses instead of square brackets, such as `(1.23, 4.56)`.
- Variants without argument: `<"Foo">`.
- Variants with one argument: `<"Bar": 123>`.
- Unquoted field names and variants are accepted if they match the pattern `[A-Za-z][A-Za-z_0-9]*`: `{ x: <Foo>, "#y": <Bar2> }`.
- Comments: `/* multiline comment */` and `// end-of-line comment`.
- Special numeric entities: `[ Infinity, -Infinity, NaN ]`.

# Parsing JSON

Parse JSON data.

```rust
use yojson_rs;
fn main () {
  let json = r#"
    {
      x : 123,
      y : {
            "y1" : "abc\ndef\u0021",
            "y2" : [null, 123.45, (12, "y3")]
          },
      z : NaN,
    }
    "#;
  assert!(yojson_rs::parser::parse(json).is_ok());
}
```

# Convert to a JSON string.

A data structure can be converted to a JSON string by `to_string`.

```rust
use yojson_rs;
fn main() {
  let json_str = r#"
    {
      x : 123,
      y : {
             "y1" : "abc\ndef\u0021",
             "y2" : [null, 123.45, (12, "y3")]
          },
      z : NaN,
    }
    "#;
  let json = yojson_rs::parser::parse(json_str).unwrap();
  println!("{}", yojson_rs::to_string(json));
}
```

---

(c) 2021 Naoki Kaneko (a.k.a. "puripuri2100")
