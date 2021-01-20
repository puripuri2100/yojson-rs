extern crate yojson_rs;

#[cfg(test)]
mod tests {
  #[test]
  fn check() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge".to_string(),
      yojson_rs::value::Value::String("fuga".to_string()),
    );
    assert_eq!(
      yojson_rs::parser::parse(r#"{hoge : "fuga"}"#),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }

  #[test]
  fn check_escape() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge".to_string(),
      yojson_rs::value::Value::String("a\n\\b".to_string()),
    );
    assert_eq!(
      yojson_rs::parser::parse(r#"{hoge : "a\n\\b"}"#),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }

  #[test]
  fn check_unicode() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge".to_string(),
      yojson_rs::value::Value::String("a!".to_string()),
    );
    assert_eq!(
      yojson_rs::parser::parse(r#"{hoge : "a\u0021"}"#),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }

  #[test]
  fn check_name() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge\nfuga".to_string(),
      yojson_rs::value::Value::String("a!".to_string()),
    );
    assert_eq!(
      yojson_rs::parser::parse(r#"{"hoge\nfuga" : "a\u0021"}"#),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }
}
