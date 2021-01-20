extern crate yojson_rs;

#[cfg(test)]
mod tests {
  #[test]
  fn check_1() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge".to_string(),
      yojson_rs::value::Value::Variant(("Foo".to_string(), None)),
    );
    assert_eq!(
      yojson_rs::parser::parse("{hoge : <Foo>}"),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }

  #[test]
  fn check_2() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge".to_string(),
      yojson_rs::value::Value::Variant((
        "Foo".to_string(),
        Some(Box::new(yojson_rs::value::Value::Integer(123))),
      )),
    );
    assert_eq!(
      yojson_rs::parser::parse(r#"{hoge : <"Foo": 123>}"#),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }

  #[test]
  fn check_3() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge".to_string(),
      yojson_rs::value::Value::Variant((
        "Foo2".to_string(),
        Some(Box::new(yojson_rs::value::Value::Integer(123))),
      )),
    );
    assert_eq!(
      yojson_rs::parser::parse(r#"{hoge : <Foo2: 123>}"#),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }
}
