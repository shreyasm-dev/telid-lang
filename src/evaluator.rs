use crate::{
  ast::{Expression, Statement},
  error::EvaluationError,
};
use scoped_stack::ScopedStack;
use std::collections::HashMap;
use strum_macros::AsRefStr;

pub type Scope<T> = ScopedStack<String, T>;

pub fn evaluate(
  program: Vec<Statement>,
  mut scope: Scope<Variable>,
) -> Result<Scope<Variable>, EvaluationError> {
  for statement in program {
    evaluate_statement(statement, &mut scope)?;
  }
  Ok(scope)
}

fn evaluate_statement(
  statement: Statement,
  mut scope: &mut Scope<Variable>,
) -> Result<Value, EvaluationError> {
  match statement {
    Statement::Block(statements) => {
      scope.push_scope();
      let mut value = Value::Void;
      for statement in statements {
        value = evaluate_statement(statement, &mut scope)?;
      }
      scope.pop_scope();
      Ok(value)
    }
    Statement::LetStatement {
      name,
      value,
      constant,
    } => {
      let value = evaluate_expression(value, &mut scope)?;
      scope.insert(
        name.0,
        Variable {
          kind: value.clone(),
          constant,
        },
      );
      Ok(value)
    }
    _ => todo!(),
  }
}

fn evaluate_expression(
  expression: Expression,
  mut scope: &mut Scope<Variable>,
) -> Result<Value, EvaluationError> {
  match expression {
    Expression::Void => Ok(Value::Void),
    Expression::Identifier(identifier) => match scope.get(&identifier.0) {
      Some(variable) => Ok(variable.kind.clone()),
      None => Err(EvaluationError::UndefinedVariable(identifier.0)),
    },
    Expression::NumberLiteral(number) => Ok(Value::Number(number)),
    Expression::StringLiteral(string) => Ok(Value::String(string)),
    Expression::BooleanLiteral(boolean) => Ok(Value::Boolean(boolean)),
    Expression::ArrayLiteral(expressions) => {
      let mut array = Vec::new();
      for expression in expressions {
        array.push(evaluate_expression(expression, &mut scope)?);
      }
      Ok(Value::Array(array))
    }
    Expression::Index { iterable, index } => {
      let iterable = evaluate_expression(*iterable, &mut scope)?;
      let index = evaluate_expression(*index, &mut scope)?;
      match (iterable.clone(), index) {
        (Value::Array(array), Value::Number(number)) => {
          let index = number as usize;
          if index >= array.len() {
            Err(EvaluationError::IndexOutOfBounds(index, array.len()))
          } else {
            Ok(array[index].clone())
          }
        }
        _ => Err(EvaluationError::InvalidType(
          iterable.as_ref().to_string(),
          vec!["Array".to_string()],
        )),
      }
    }
    _ => todo!(),
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
  kind: Value,
  constant: bool,
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
}
