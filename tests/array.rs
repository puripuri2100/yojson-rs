extern crate yojson_rs;

#[cfg(test)]
mod tests {
  #[test]
  fn check_1() {
    let mut assoc2 = std::collections::HashMap::new();
    assoc2.insert("fuga".to_string(), yojson_rs::value::Value::Integer(23));
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge".to_string(),
      yojson_rs::value::Value::Array(vec![
        yojson_rs::value::Value::Null,
        yojson_rs::value::Value::Integer(123),
        yojson_rs::value::Value::Assoc(assoc2),
      ]),
    );
    assert_eq!(
      yojson_rs::parser::parse("{hoge : [null, 123, {fuga: 23}]}"),
      Ok(yojson_rs::value::Value::Assoc(assoc))
    );
  }

  #[test]
  fn check_2() {
    let mut assoc2 = std::collections::HashMap::new();
    assoc2.insert("fuga".to_string(), yojson_rs::value::Value::Integer(23));
    let mut assoc = std::collections::HashMap::new();
    assoc.insert(
      "hoge".to_string(),
      yojson_rs::value::Value::Array(vec![
        yojson_rs::value::Value::Null,
        yojson_rs::value::Value::Integer(123),
        yojson_rs::value::Value::Assoc(assoc2),
      ]),
    );
    assert!(
      yojson_rs::parser::parse("{hoge : [null, 123, {fuga: 23}, ]}").is_err()
    );
  }
}

#[test]
fn check_3() {
  let mut assoc2 = std::collections::HashMap::new();
  assoc2.insert("fuga".to_string(), yojson_rs::value::Value::Integer(23));
  let mut assoc = std::collections::HashMap::new();
  assoc.insert(
    "hoge".to_string(),
    yojson_rs::value::Value::Array(vec![
      yojson_rs::value::Value::Null,
      yojson_rs::value::Value::Integer(123),
      yojson_rs::value::Value::Assoc(assoc2),
    ]),
  );
  assert!(
    yojson_rs::parser::parse("{hoge : [null, 123, {fuga: 23}],}").is_err()
  );
}
