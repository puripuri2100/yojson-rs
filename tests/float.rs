extern crate yojson_rs;

#[cfg(test)]
mod tests {
  #[test]
  fn check_parse_infinity() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge".to_string(),
      yojson_rs::value::Value::Float(std::f64::INFINITY),
    );
    assert_eq!(
      yojson_rs::parser::parse(r#"{hoge : Infinity}"#),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }

  #[test]
  fn check_parse_neg_infinity() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge".to_string(),
      yojson_rs::value::Value::Float(std::f64::NEG_INFINITY),
    );
    assert_eq!(
      yojson_rs::parser::parse(r#"{hoge : -Infinity}"#),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }

  #[test]
  fn check() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge".to_string(),
      yojson_rs::value::Value::Float(123.45),
    );
    assert_eq!(
      yojson_rs::parser::parse(r#"{hoge : 123.45}"#),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }
}
