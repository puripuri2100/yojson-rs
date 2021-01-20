extern crate yojson_rs;

#[cfg(test)]
mod tests {
  #[test]
  fn check_null() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert("hoge".to_string(), yojson_rs::value::Value::Null);
    assert_eq!(
      yojson_rs::parser::parse("{hoge : null}"),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }

  #[test]
  fn check_null_2() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert("hoge".to_string(), yojson_rs::value::Value::Null);
    assoc.insert("fuga".to_string(), yojson_rs::value::Value::Null);
    assert_eq!(
      yojson_rs::parser::parse(r#"{hoge : null, "fuga": null}"#),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }
  #[test]
  fn check_null_lst() {
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge".to_string(),
      yojson_rs::value::Value::Array(vec![
        yojson_rs::value::Value::Null,
        yojson_rs::value::Value::Null,
        yojson_rs::value::Value::Null,
      ]),
    );
    assert_eq!(
      yojson_rs::parser::parse("{hoge : [null, null, null]}"),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }
}
