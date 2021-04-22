//! # Yojson parser
//! [![crates.io][crates-badge]][crates]
//! [![docs.rs][docs-badge]][docs]
//! [![Build Status][ci-badge]][ci]
//! [![source badge][source-badge]][source]
//! [![license badge][license-badge]][license]
//!
//! [crates]: https://crates.io/crates/yojson-rs
//! [crates-badge]: https://img.shields.io/crates/v/yojson-rs
//! [docs]: https://docs.rs/yojson-rs/
//! [docs-badge]: https://img.shields.io/badge/docs.rs-yojson_rs-blue
//! [ci]: https://github.com/puripuri2100/yojson-rs/actions?query=workflow%3ACI
//! [ci-badge]: https://github.com/puripuri2100/yojson-rs/workflows/CI/badge.svg?branch=master
//! [source]: https://github.com/puripuri2100/yojson-rs
//! [source-badge]: https://img.shields.io/badge/source-github-blue
//! [license]: https://github.com/puripuri2100/yojson-rs/blob/master/LICENSE
//! [license-badge]: https://img.shields.io/badge/license-MIT-blue
//!
//! This library parses [JSON data (yojson format)](https://mjambon.github.io/mjambon2016/yojson.html) into a nested Rust tree data structure.
//!
//! # Yojson values
//!
//! A value in Yojson is represented with the `Value` enum in this crate:
//!
//! ```ignore
//! pub enum Value {
//!   Null,
//!   Bool(bool),
//!   Integer(i64),
//!   Float(f64),
//!   String(String),
//!   Assoc(Assoc),
//!   Array(Array),
//!   Tuple(Vec<Value>),
//!   Variant(Variant),
//! }
//! ```
//!
//! The Yojson format is an extension of the JSON format. See ["Yojson format document"](https://mjambon.github.io/mjambon2016/yojson.html) for more information.
//! - Tuples: like JSON arrays but within parentheses instead of square brackets, such as `(1.23, 4.56)`.
//! - Variants without argument: `<"Foo">`.
//! - Variants with one argument: `<"Bar": 123>`.
//! - Unquoted field names and variants are accepted if they match the pattern `[A-Za-z][A-Za-z_0-9]*`: `{ x: <Foo>, "#y": <Bar2> }`.
//! - Comments: `/* multiline comment */` and `// end-of-line comment`.
//! - Special numeric entities: `[ Infinity, -Infinity, NaN ]`.
//!
//! # Parsing JSON
//!
//! Parse JSON data.
//!
//! ```ignore
//! use yojson_rs;
//!
//! # fn main () {
//! let json = r#"
//!   {
//!     x : 123,
//!     y : {
//!            "y1" : "abc\ndef\u0021",
//!            "y2" : [null, 123.45, (12, "y3")]
//!         },
//!     z : NaN
//!   }
//!   "#;
//! assert!(yojson_rs::parser::parse(json).is_ok());
//! # }
//!
//! ```
//!
//! # Convert to a JSON string.
//! A data structure can be converted to a JSON string by `to_string`.
//!
//! ```ignore
//! use yojson_rs;
//!
//! # fn main() {
//! let json_str = r#"
//!   {
//!     x : 123,
//!     y : {
//!            "y1" : "abc\ndef\u0021",
//!            "y2" : [null, 123.45, (12, "y3")]
//!         },
//!     z : NaN
//!   }
//!   "#;
//! let json = yojson_rs::parser::parse(json_str).unwrap();
//! println!("{}", yojson_rs::to_string(json));
//! # }
//! ```

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;
pub mod value;

/// Convert to a JSON string.
pub fn to_string(value: value::Value) -> String {
  match value {
    value::Value::Null => "null".to_string(),
    value::Value::Bool(b) => b.to_string(),
    value::Value::Integer(i) => i.to_string(),
    value::Value::Float(f) => f.to_string(),
    value::Value::String(s) => format!("{:?}", s),
    value::Value::Assoc(assoc) => {
      let mut s = String::new();
      for (i, item) in assoc.iter().enumerate() {
        let (name, value) = item;
        if i == 0 {
          s.push_str(&format!("{}:{}", name, to_string(value.clone())))
        } else {
          s.push_str(&format!(",{}:{}", name, to_string(value.clone())))
        }
      }
      format!("{{{}}}", s)
    }
    value::Value::Array(array) => {
      let mut s = String::new();
      for (i, item) in array.iter().enumerate() {
        if i == 0 {
          s.push_str(&to_string(item.clone()))
        } else {
          s.push_str(&format!(",{}", to_string(item.clone())))
        }
      }
      format!("[{}]", s)
    }
    value::Value::Tuple(tuple) => {
      let mut s = String::new();
      for (i, item) in tuple.iter().enumerate() {
        if i == 0 {
          s.push_str(&to_string(item.clone()))
        } else {
          s.push_str(&format!(",{}", to_string(item.clone())))
        }
      }
      format!("({})", s)
    }
    value::Value::Variant(variant) => {
      let (name, value_opt) = variant;
      match value_opt {
        None => format!("<{}>", name),
        Some(value) => format!("<{}:{}>", name, to_string(*value)),
      }
    }
  }
}
