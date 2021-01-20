//! Definition of a parse fucntion.

use crate::pest::Parser;
use pest::iterators::Pair;
use std::collections::HashMap;

use super::value;

#[derive(Parser)]
#[grammar = "parser.pest"]
struct YojsonParser;

pub fn parse(text: &str) -> Result<value::Value, pest::error::Error<Rule>> {
  let json = YojsonParser::parse(Rule::json, text)?.next().unwrap();
  let value = parse_value(json);
  Ok(value)
}

fn parse_value(pair: Pair<Rule>) -> value::Value {
  match pair.as_rule() {
    Rule::null => value::Value::Null,
    Rule::bool => value::Value::Bool(pair.as_str().parse().unwrap()),
    Rule::integer => value::Value::Integer(pair.as_str().parse().unwrap()),
    Rule::float => value::Value::Float(pair.as_str().parse().unwrap()),
    Rule::string => {
      let str = parse_string(pair.into_inner().next().unwrap());
      value::Value::String(str)
    }
    Rule::assoc => {
      let mut assoc = HashMap::new();
      pair.into_inner().for_each(|pair| {
        let mut inner_rules = pair.into_inner();
        let name = parse_name(inner_rules.next().unwrap());
        let value = parse_value(inner_rules.next().unwrap());
        assoc.insert(name, value);
      });
      value::Value::Assoc(assoc)
    }
    Rule::array => value::Value::Array(pair.into_inner().map(parse_value).collect()),
    Rule::tuple => value::Value::Tuple(pair.into_inner().map(parse_value).collect()),
    Rule::variant => {
      let mut inner_rules = pair.into_inner();
      let name = parse_name(inner_rules.next().unwrap());
      let value = match inner_rules.next() {
        None => None,
        Some(rule) => Some(Box::new(parse_value(rule))),
      };
      let variant = (name, value);
      value::Value::Variant(variant)
    }
    Rule::json
    | Rule::EOI
    | Rule::pair
    | Rule::value
    | Rule::inner
    | Rule::name
    | Rule::ascii_inner
    | Rule::quoted_string
    | Rule::c
    | Rule::escape_char
    | Rule::unicode_char
    | Rule::unquoted_string
    | Rule::ascii_char
    | Rule::WHITESPACE
    | Rule::COMMENT => unreachable!(),
  }
}

fn parse_name(pair: Pair<Rule>) -> String {
  let mut inner_rules = pair.into_inner().peekable();
  let str = match inner_rules.peek().unwrap().as_rule() {
    Rule::ascii_inner => inner_rules.next().unwrap().as_str().to_string(),
    Rule::inner => {
      let mut s = String::new();
      for pair in inner_rules.next().unwrap().into_inner() {
        match pair.as_rule() {
          Rule::c => s.push_str(pair.as_str()),
          Rule::escape_char => {
            let c = match pair.as_str().chars().nth(1).unwrap() {
              '"' => '\"',
              '\\' => '\\',
              '/' => '/',
              'b' => '\u{0008}', // Backspace
              'f' => '\u{000c}', // Form Feed
              'n' => '\n',
              'r' => '\r',
              't' => '\t',
              _ => unreachable!(),
            };
            s.push(c)
          }
          Rule::unicode_char => {
            let hex = pair.as_str().chars().skip(2).collect::<String>();
            let hex_i64 = i64::from_str_radix(&hex, 16).unwrap();
            let str = String::from_utf8(vec![hex_i64 as u8]).unwrap();
            s.push_str(&str)
          }
          _ => break,
        }
      }
      s
    }
    _ => unreachable!(),
  };
  str
}

fn parse_string(pair: Pair<Rule>) -> String {
  let mut s = String::new();
  let inner_rules = pair.into_inner();
  for pair in inner_rules {
    match pair.as_rule() {
      Rule::c => s.push_str(pair.as_str()),
      Rule::escape_char => {
        let c = match pair.as_str().chars().nth(1).unwrap() {
          '"' => '\"',
          '\\' => '\\',
          '/' => '/',
          'b' => '\u{0008}', // Backspace
          'f' => '\u{000c}', // Form Feed
          'n' => '\n',
          'r' => '\r',
          't' => '\t',
          _ => unreachable!(),
        };
        s.push(c)
      }
      Rule::unicode_char => {
        let hex = pair.as_str().chars().skip(2).collect::<String>();
        let hex_i64 = i64::from_str_radix(&hex, 16).unwrap();
        let str = String::from_utf8(vec![hex_i64 as u8]).unwrap();
        s.push_str(&str)
      }
      _ => break,
    }
  }
  s
}
