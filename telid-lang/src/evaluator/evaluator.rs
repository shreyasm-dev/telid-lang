use super::{
  scope::Scope,
  value::{Value, Variable},
};
use crate::{
  error::EvaluationError,
  parser::ast::{BinaryOperator, Expression, Statement, UnaryOperator},
};

pub fn evaluate(
  program: Vec<Statement>,
  mut scope: Scope,
) -> Result<(Value, Scope), EvaluationError> {
  let mut value = Value::Void;
  for statement in program {
    value = evaluate_statement(statement, &mut scope)?;
  }
  Ok((value, scope))
}

fn evaluate_statement(
  statement: Statement,
  mut scope: &mut Scope,
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
      match scope.clone().get(&name.0) {
        Some(variable) => {
          if variable.constant {
            return Err(EvaluationError::ConstantReassignment(name.0));
          }
        }
        None => {}
      }

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
    Statement::Assignment { name, value } => match scope.clone().get(&name.0) {
      Some(variable) => {
        let value = evaluate_expression(value, &mut scope)?;

        if variable.constant {
          Err(EvaluationError::ConstantReassignment(name.0))
        } else {
          scope.insert_existing(
            name.0,
            Variable {
              value: value.clone(),
              constant: false,
            },
          );
          Ok(value)
        }
      }
      None => Err(EvaluationError::UndefinedVariable(name.0)),
    },
  }
}

fn evaluate_expression(
  expression: Expression,
  mut scope: &mut Scope,
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
        // any type, any type
        (BinaryOperator::Equal, left, right) => Ok(Value::Boolean(left == right)),
        (BinaryOperator::NotEqual, left, right) => Ok(Value::Boolean(left != right)),

        // number, number
        (BinaryOperator::Add, Value::Number(left), Value::Number(right)) => {
          Ok(Value::Number(left + right))
        }
        (BinaryOperator::Subtract, Value::Number(left), Value::Number(right)) => {
          Ok(Value::Number(left - right))
        }
        (BinaryOperator::Multiply, Value::Number(left), Value::Number(right)) => {
          Ok(Value::Number(left * right))
        }
        (BinaryOperator::Divide, Value::Number(left), Value::Number(right)) => {
          Ok(Value::Number(left / right))
        }
        (BinaryOperator::Modulo, Value::Number(left), Value::Number(right)) => {
          Ok(Value::Number(left % right))
        }
        (BinaryOperator::LessThan, Value::Number(left), Value::Number(right)) => {
          Ok(Value::Boolean(left < right))
        }
        (BinaryOperator::LessThanOrEqual, Value::Number(left), Value::Number(right)) => {
          Ok(Value::Boolean(left <= right))
        }
        (BinaryOperator::GreaterThan, Value::Number(left), Value::Number(right)) => {
          Ok(Value::Boolean(left > right))
        }
        (BinaryOperator::GreaterThanOrEqual, Value::Number(left), Value::Number(right)) => {
          Ok(Value::Boolean(left >= right))
        }
        (BinaryOperator::Range, Value::Number(left), Value::Number(right)) => {
          if left > right {
            return Err(EvaluationError::InvalidRange(left, right));
          }

          let mut array = Vec::new();
          for i in left as usize..=right as usize {
            array.push(Value::Number(i as f64));
          }
          Ok(Value::Array(array))
        }

        // string, string
        (BinaryOperator::LessThan, Value::String(left), Value::String(right)) => {
          Ok(Value::Boolean(left < right))
        }
        (BinaryOperator::LessThanOrEqual, Value::String(left), Value::String(right)) => {
          Ok(Value::Boolean(left <= right))
        }
        (BinaryOperator::GreaterThan, Value::String(left), Value::String(right)) => {
          Ok(Value::Boolean(left > right))
        }
        (BinaryOperator::GreaterThanOrEqual, Value::String(left), Value::String(right)) => {
          Ok(Value::Boolean(left >= right))
        }

        // boolean, boolean
        (BinaryOperator::And, Value::Boolean(left), Value::Boolean(right)) => {
          Ok(Value::Boolean(left && right))
        }
        (BinaryOperator::Or, Value::Boolean(left), Value::Boolean(right)) => {
          Ok(Value::Boolean(left || right))
        }

        // string, any type
        (BinaryOperator::Add, Value::String(left), right) => {
          Ok(Value::String(format!("{}{}", left, right.to_string())))
        }
        (BinaryOperator::Add, left, Value::String(right)) => {
          Ok(Value::String(format!("{}{}", left.to_string(), right)))
        }

        // unhandled cases
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
    Expression::While { condition, body } => {
      let mut value = Vec::new();

      loop {
        let condition = evaluate_expression(*condition.clone(), &mut scope)?;
        match condition {
          Value::Boolean(boolean) => {
            if boolean {
              scope.push_scope();
              value.push(evaluate_statement(*body.clone(), &mut scope)?);
              scope.pop_scope();
            } else {
              break;
            }
          }
          _ => {
            return Err(EvaluationError::InvalidType(
              condition.as_ref().to_string(),
              vec!["Boolean".to_string()],
            ))
          }
        }
      }

      Ok(Value::Array(value))
    }
  }
}
