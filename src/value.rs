//! Definition of a Yojson value

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::mem::discriminant;

/// Representation of a Yojson value.
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Value {
  /// JSON null
  Null,
  /// JSON boolean
  Bool(bool),
  /// JSON number without decimal point or exponent.
  Integer(i64),
  /// JSON number, Infinity, -Infinity or NaN.
  Float(f64),
  /// JSON string.
  String(String),
  /// JSON object.
  Assoc(Assoc),
  /// JSON array
  Array(Array),
  /// Tuple (non-standard extension of JSON). Syntax: `("abc", 123)`.
  Tuple(Vec<Value>),
  /// Variant (non-standard extension of JSON). Syntax: `<"Foo">` or `<"Bar":123>`.
  Variant(Variant),
}

/// JSON object.
pub type Assoc = HashMap<String, Value>;

/// JSON array
pub type Array = Vec<Value>;

/// Variant (non-standard extension of JSON). Syntax: `<"Foo">` or `<"Bar":123>`.
pub type Variant = (String, Option<Box<Value>>);

impl Value {
  /// Tests whether this value is a null.
  pub fn is_null(&self) -> bool {
    match self {
      Value::Null => true,
      _ => false,
    }
  }

  /// Extracts the integer value if it is an integer.
  pub fn as_integer(&self) -> Option<i64> {
    match *self {
      Value::Integer(i) => Some(i),
      _ => None,
    }
  }

  /// Tests whether this value is an integer.
  pub fn is_integer(&self) -> bool {
    self.as_integer().is_some()
  }

  /// Extracts the float value if it is a float.
  pub fn as_float(&self) -> Option<f64> {
    match *self {
      Value::Float(f) => Some(f),
      _ => None,
    }
  }

  /// Tests whether this value is a float.
  pub fn is_float(&self) -> bool {
    self.as_float().is_some()
  }

  /// Extracts the boolean value if it is a boolean.
  pub fn as_bool(&self) -> Option<bool> {
    match *self {
      Value::Bool(b) => Some(b),
      _ => None,
    }
  }

  /// Tests whether this value is a boolean.
  pub fn is_bool(&self) -> bool {
    self.as_bool().is_some()
  }

  /// Extracts the string of this value if it is a string.
  pub fn as_str(&self) -> Option<&str> {
    match *self {
      Value::String(ref s) => Some(&**s),
      _ => None,
    }
  }

  /// Tests if this value is a string.
  pub fn is_str(&self) -> bool {
    self.as_str().is_some()
  }

  /// Extracts the array value if it is an array.
  pub fn as_array(&self) -> Option<&Vec<Value>> {
    match *self {
      Value::Array(ref s) => Some(s),
      _ => None,
    }
  }

  /// Extracts the array value if it is an array.
  pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
    match *self {
      Value::Array(ref mut s) => Some(s),
      _ => None,
    }
  }

  /// Tests whether this value is an array.
  pub fn is_array(&self) -> bool {
    self.as_array().is_some()
  }

  /// Extracts the list values if it is a tuple.
  pub fn as_tuple(&self) -> Option<&Vec<Value>> {
    match *self {
      Value::Tuple(ref s) => Some(s),
      _ => None,
    }
  }

  /// Extracts the list values if it is a tuple.
  pub fn as_tuple_mut(&mut self) -> Option<&mut Vec<Value>> {
    match *self {
      Value::Tuple(ref mut s) => Some(s),
      _ => None,
    }
  }

  /// Tests whether this value is a tuple.
  pub fn is_tuple(&self) -> bool {
    self.as_tuple().is_some()
  }

  /// Extracts the objects value if it is an assoc.
  pub fn as_assoc(&self) -> Option<&HashMap<String, Value>> {
    match *self {
      Value::Assoc(ref s) => Some(s),
      _ => None,
    }
  }

  /// Extracts the objects value if it is an assoc.
  pub fn as_assoc_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
    match *self {
      Value::Assoc(ref mut s) => Some(s),
      _ => None,
    }
  }

  /// Tests whether this value is an assoc.
  pub fn is_assoc(&self) -> bool {
    self.as_assoc().is_some()
  }

  /// Extracts the variant value if it is a variant.
  pub fn as_variant(&self) -> Option<&Variant> {
    match *self {
      Value::Variant(ref s) => Some(s),
      _ => None,
    }
  }

  /// Extracts the variant value if it is a variant.
  pub fn as_variant_mut(&mut self) -> Option<&mut Variant> {
    match *self {
      Value::Variant(ref mut s) => Some(s),
      _ => None,
    }
  }

  /// Tests whether this value is a variant.
  pub fn is_variant(&self) -> bool {
    self.as_variant().is_some()
  }

  /// Tests whether this and another value have the same type.
  pub fn same_type(&self, other: &Value) -> bool {
    discriminant(self) == discriminant(other)
  }

  /// Returns a human-readable representation of the type of this value.
  pub fn type_str(&self) -> &'static str {
    match *self {
      Value::Null => "null",
      Value::String(..) => "string",
      Value::Integer(..) => "integer",
      Value::Float(..) => "float",
      Value::Bool(..) => "boolean",
      Value::Array(..) => "array",
      Value::Assoc(..) => "assoc",
      Value::Tuple(..) => "tuple",
      Value::Variant(..) => "variant",
    }
  }
}
