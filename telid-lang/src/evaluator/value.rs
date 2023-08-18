use crate::{error::EvaluationError, parser::ast::Statement};
use std::ops::Range;
use strum_macros::AsRefStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
  pub value: Value,
  pub constant: bool,
}

#[derive(Debug, Clone, PartialEq, AsRefStr)]
pub enum Value {
  Void,
  Number(f64),
  String(String),
  Boolean(bool),
  Array(Vec<Value>),
  Function {
    parameters: Vec<String>,
    body: Box<Statement>,
  },
  RustFunction {
    parameter_count: usize,
    function: fn(Range<usize>, Vec<Value>) -> Result<Value, EvaluationError>,
  },
}

impl ToString for Value {
  fn to_string(&self) -> String {
    match self {
      Value::Void => "void".to_string(),
      Value::Number(number) => number.to_string(),
      Value::String(string) => string.clone(),
      Value::Boolean(boolean) => boolean.to_string(),
      Value::Array(array) => format!(
        "[{}]",
        array
          .iter()
          .map(|value| value.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Value::Function {
        parameters,
        body: _,
      } => {
        let mut string = String::from("fn (");
        for parameter in parameters {
          string.push_str(&parameter);
          string.push_str(", ");
        }
        string.push_str(") ");
        string
      }
      Value::RustFunction {
        parameter_count, ..
      } => {
        format!("RustFn({})", parameter_count)
      }
    }
  }
}
