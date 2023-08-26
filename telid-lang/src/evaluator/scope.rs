use super::{
  util::error,
  value::{Value, Variable},
};
use crate::error::EvaluationErrorKind;
use scoped_stack::ScopedStack;
use std::io::{stdin, stdout, Write};

pub type Scope = ScopedStack<String, Variable>;

// TODO: Pass spans to all Rust functions

pub fn default() -> Scope {
  let mut scope = Scope::new();

  scope.insert(
    String::from("println"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 1,
        function: |_, parameters| {
          println!("{}", parameters[0].to_string());
          Ok(Value::Void)
        },
      },
      constant: true,
    },
  );

  scope.insert(
    String::from("print"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 1,
        function: |_, parameters| {
          print!("{}", parameters[0].to_string());
          stdout().flush().unwrap();
          Ok(Value::Void)
        },
      },
      constant: true,
    },
  );

  scope.insert(
    String::from("exit"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 1,
        function: |span, parameters| match parameters[0] {
          Value::Number(code) => std::process::exit(code as i32),
          _ => error(
            EvaluationErrorKind::InvalidType(
              parameters[0].as_ref().to_string(),
              vec![String::from("Number")],
            ),
            span,
          ),
        },
      },
      constant: true,
    },
  );

  scope.insert(
    String::from("readln"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 0,
        function: |_, _| {
          let mut input = String::new();
          stdin().read_line(&mut input).expect("Failed to read line");
          Ok(Value::String(input.trim().to_string()))
        },
      },
      constant: true,
    },
  );

  scope.insert(
    String::from("assert"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 1,
        function: |span, parameters| match parameters[0] {
          Value::Boolean(true) => Ok(Value::Void),
          Value::Boolean(false) => error(EvaluationErrorKind::AssertionFailed, span),
          _ => error(
            EvaluationErrorKind::InvalidType(
              parameters[0].as_ref().to_string(),
              vec![String::from("Boolean")],
            ),
            span,
          ),
        },
      },
      constant: true,
    },
  );

  scope.insert(
    String::from("parse"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 1,
        function: |span, parameters| match &parameters[0] {
          Value::String(string) => match string.parse() {
            Ok(number) => Ok(Value::Number(number)),
            Err(_) => Ok(Value::Void),
          },
          _ => error(
            EvaluationErrorKind::InvalidType(
              parameters[0].as_ref().to_string(),
              vec![String::from("String")],
            ),
            span,
          ),
        },
      },
      constant: true,
    },
  );

  scope.insert(
    String::from("type"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 1,
        function: |_, parameters| Ok(Value::String(parameters[0].as_ref().to_string())),
      },
      constant: true,
    },
  );

  scope.insert(
    String::from("len"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 1,
        function: |span, parameters| match &parameters[0] {
          Value::String(string) => Ok(Value::Number(string.len() as f64)),
          Value::Array(array) => Ok(Value::Number(array.len() as f64)),
          _ => error(
            EvaluationErrorKind::InvalidType(
              parameters[0].as_ref().to_string(),
              vec![String::from("String"), String::from("Array")],
            ),
            span,
          ),
        },
      },
      constant: true,
    },
  );

  scope.insert(
    String::from("filter"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 2,
        function: |span, parameters| match &parameters[0] {
          Value::Array(array) => match &parameters[1] {
            Value::String(string) => {
              let mut result = Vec::new();
              for element in array {
                if element.as_ref() != string {
                  result.push(element.clone());
                }
              }

              Ok(Value::Array(result))
            }
            _ => error(
              EvaluationErrorKind::InvalidType(
                parameters[1].as_ref().to_string(),
                vec![String::from("String")],
              ),
              span,
            ),
          },
          _ => error(
            EvaluationErrorKind::InvalidType(
              parameters[0].as_ref().to_string(),
              vec![String::from("Array")],
            ),
            span,
          ),
        },
      },
      constant: true,
    },
  );

  scope.insert(
    String::from("concat"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 2,
        function: |span, parameters| match &parameters[0] {
          Value::Array(array) => match &parameters[1] {
            Value::Array(other) => {
              let mut result = Vec::new();
              for element in array {
                result.push(element.clone());
              }
              for element in other {
                result.push(element.clone());
              }

              Ok(Value::Array(result))
            }
            _ => error(
              EvaluationErrorKind::InvalidType(
                parameters[1].as_ref().to_string(),
                vec![String::from("Array")],
              ),
              span,
            ),
          },
          _ => error(
            EvaluationErrorKind::InvalidType(
              parameters[0].as_ref().to_string(),
              vec![String::from("Array")],
            ),
            span,
          ),
        },
      },
      constant: true,
    },
  );

  scope
}
