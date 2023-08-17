use crate::{
  ast::{Expression, Statement},
  error::EvaluationError,
};
use scoped_stack::ScopedStack;
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
          value: value.clone(),
          constant,
        },
      );
      Ok(value)
    }
    Statement::ExpressionStatement(expression) => evaluate_expression(expression, &mut scope),
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
      Some(variable) => Ok(variable.value.clone()),
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
    // name: Identifier, parameters: Vec<Expression>
    Expression::FunctionCall { name, parameters } => {
      let function = match scope.get(&name.0) {
        Some(variable) => variable.value.clone(),
        None => return Err(EvaluationError::UndefinedVariable(name.0)),
      };

      match function {
        Value::RustFunction {
          parameter_count,
          function,
        } => {
          if parameters.len() != parameter_count {
            return Err(EvaluationError::IncorrectParameterCount(
              parameters.len(),
              parameter_count,
            ));
          }
          let mut passed_parameters: Vec<Value> = Vec::new();
          for parameter in parameters {
            passed_parameters.push(evaluate_expression(parameter, &mut scope)?);
          }
          function(passed_parameters)
        }
        _ => todo!(),
      }
    }
    _ => todo!(),
  }
}

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
    function: fn(Vec<Value>) -> Result<Value, EvaluationError>,
  },
}

impl ToString for Value {
  fn to_string(&self) -> String {
    match self {
      Value::Void => "Void".to_string(),
      Value::Number(number) => number.to_string(),
      Value::String(string) => string.clone(),
      Value::Boolean(boolean) => boolean.to_string(),
      Value::Array(array) => {
        let mut string = String::from("[");
        for value in array {
          string.push_str(&value.to_string());
          string.push_str(", ");
        }
        string.push_str("]");
        string
      }
      Value::Function { parameters, body: _ } => {
        let mut string = String::from("fn (");
        for parameter in parameters {
          string.push_str(&parameter);
          string.push_str(", ");
        }
        string.push_str(") ");
        string
      }
      Value::RustFunction { parameter_count, .. } => {
        format!("RustFn({})", parameter_count)
      }
    }
  }
}
