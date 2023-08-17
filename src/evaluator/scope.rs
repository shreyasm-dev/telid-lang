use crate::error::EvaluationError;

use super::value::{Value, Variable};
use scoped_stack::ScopedStack;
use std::io::{stdin, stdout, Write};

pub type Scope = ScopedStack<String, Variable>;

pub fn default() -> Scope {
  let mut scope = Scope::new();

  scope.insert(
    String::from("println"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 1,
        function: |parameters| {
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
        function: |parameters| {
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
        function: |parameters| match parameters[0] {
          Value::Number(code) => std::process::exit(code as i32),
          _ => Err(EvaluationError::InvalidType(
            parameters[0].as_ref().to_string(),
            vec![String::from("Number")],
          )),
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
        function: |_| {
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
        function: |parameters| {
          match parameters[0] {
            Value::Boolean(true) => Ok(Value::Void),
            Value::Boolean(false) => Err(EvaluationError::AssertionFailed),
            _ => Err(EvaluationError::InvalidType(
              parameters[0].as_ref().to_string(),
              vec![String::from("Boolean")],
            )),
          }
        },
      },
      constant: true,
    },
  );

  scope
}
