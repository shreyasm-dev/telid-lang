use crate::{
  ast::{BinaryOperator, Expression, Statement, UnaryOperator},
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
    Statement::FunctionDeclaration {
      name,
      parameters,
      body,
    } => {
      scope.insert(
        name.0,
        Variable {
          value: Value::Function {
            parameters: parameters.iter().map(|p| p.0.clone()).collect(),
            body,
          },
          constant: false,
        },
      );
      Ok(Value::Void)
    }
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
    Expression::FunctionCall { name, arguments } => {
      let function = match scope.get(&name.0) {
        Some(variable) => variable.value.clone(),
        None => return Err(EvaluationError::UndefinedVariable(name.0)),
      };

      match function {
        Value::RustFunction {
          parameter_count,
          function,
        } => {
          if arguments.len() != parameter_count {
            return Err(EvaluationError::IncorrectParameterCount(
              arguments.len(),
              parameter_count,
            ));
          }
          let mut passed_parameters: Vec<Value> = Vec::new();
          for parameter in arguments {
            passed_parameters.push(evaluate_expression(parameter, &mut scope)?);
          }
          function(passed_parameters)
        }
        Value::Function { parameters, body } => {
          if parameters.len() != parameters.len() {
            return Err(EvaluationError::IncorrectParameterCount(
              parameters.len(),
              parameters.len(),
            ));
          }
          scope.push_scope();
          for (parameter, value) in parameters.iter().zip(arguments) {
            let value = evaluate_expression(value, &mut scope)?;
            scope.insert(
              parameter.clone(),
              Variable {
                value,
                constant: true,
              },
            );
          }
          let result = evaluate_statement(*body, &mut scope);
          scope.pop_scope();
          result
        }
        _ => Err(EvaluationError::InvalidType(
          function.as_ref().to_string(),
          vec!["Function".to_string()],
        )),
      }
    }
    Expression::Unary { operator, operand } => {
      let operand = evaluate_expression(*operand, &mut scope)?;
      match (operator.clone(), operand.clone()) {
        (UnaryOperator::Negate, Value::Number(number)) => Ok(Value::Number(-number)),
        (UnaryOperator::Not, Value::Boolean(boolean)) => Ok(Value::Boolean(!boolean)),
        _ => Err(EvaluationError::InvalidOperator(
          operator.to_string(),
          operand.as_ref().to_string(),
          "".to_string(),
        )),
      }
    }
    Expression::Binary {
      operator,
      left,
      right,
    } => {
      let left = evaluate_expression(*left, &mut scope)?;
      let right = evaluate_expression(*right, &mut scope)?;
      match (operator.clone(), left.clone(), right.clone()) {
        (operator, Value::Number(left), Value::Number(right)) => match operator {
          BinaryOperator::Add => Ok(Value::Number(left + right)),
          BinaryOperator::Subtract => Ok(Value::Number(left - right)),
          BinaryOperator::Multiply => Ok(Value::Number(left * right)),
          BinaryOperator::Divide => Ok(Value::Number(left / right)),
          BinaryOperator::Modulo => Ok(Value::Number(left % right)),
          BinaryOperator::Equal => Ok(Value::Boolean(left == right)),
          BinaryOperator::NotEqual => Ok(Value::Boolean(left != right)),
          BinaryOperator::LessThan => Ok(Value::Boolean(left < right)),
          BinaryOperator::LessThanOrEqual => Ok(Value::Boolean(left <= right)),
          BinaryOperator::GreaterThan => Ok(Value::Boolean(left > right)),
          BinaryOperator::GreaterThanOrEqual => Ok(Value::Boolean(left >= right)),
          BinaryOperator::And => Ok(Value::Boolean((left != 0.0) && (right != 0.0))),
          BinaryOperator::Or => Ok(Value::Boolean((left != 0.0) || (right != 0.0))),
        },
        (operator, Value::String(left), Value::String(right)) => match operator {
          BinaryOperator::Add => Ok(Value::String(left + &right)),
          BinaryOperator::Equal => Ok(Value::Boolean(left == right)),
          BinaryOperator::NotEqual => Ok(Value::Boolean(left != right)),
          _ => Err(EvaluationError::InvalidOperator(
            operator.to_string(),
            left.to_string(),
            right.to_string(),
          )),
        },
        (operator, left, right) => Err(EvaluationError::InvalidOperator(
          operator.to_string(),
          left.as_ref().to_string(),
          right.as_ref().to_string(),
        )),
      }
    }
    Expression::If {
      condition,
      consequence,
      alternative,
    } => {
      let condition = evaluate_expression(*condition, &mut scope)?;
      match condition {
        Value::Boolean(boolean) => {
          if boolean {
            evaluate_statement(*consequence, &mut scope)
          } else {
            match *alternative {
              Some(alternative) => evaluate_statement(alternative, &mut scope),
              None => Ok(Value::Void),
            }
          }
        }
        _ => Err(EvaluationError::InvalidType(
          condition.as_ref().to_string(),
          vec!["Boolean".to_string()],
        )),
      }
    }
    Expression::For {
      variable,
      iterable,
      body,
    } => {
      let iterable = evaluate_expression(*iterable, &mut scope)?;
      match iterable {
        Value::Array(array) => {
          let mut value = Vec::new();
          for element in array {
            scope.push_scope();
            scope.insert(
              variable.0.clone(),
              Variable {
                value: element.clone(),
                constant: true,
              },
            );
            value.push(evaluate_statement(*body.clone(), &mut scope)?);
            scope.pop_scope();
          }
          Ok(Value::Array(value))
        }
        _ => Err(EvaluationError::InvalidType(
          iterable.as_ref().to_string(),
          vec!["Array".to_string()],
        )),
      }
    }
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
