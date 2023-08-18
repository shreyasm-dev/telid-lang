use super::{
  scope::Scope,
  util::error,
  value::{Value, Variable},
};
use crate::{
  error::{EvaluationError, EvaluationErrorKind},
  parser::ast::{
    BinaryOperator, Expression, ExpressionKind, Statement, StatementKind, UnaryOperator,
  },
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
  let Statement { kind, span } = statement;

  match kind {
    StatementKind::Block(statements) => {
      scope.push_scope();
      let mut value = Value::Void;
      for statement in statements {
        value = evaluate_statement(statement, &mut scope)?;
      }
      scope.pop_scope();
      Ok(value)
    }
    StatementKind::Let {
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
    StatementKind::Expression(expression) => evaluate_expression(expression, &mut scope),
    StatementKind::FunctionDeclaration {
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
    StatementKind::Assignment { name, value } => match scope.clone().get(&name.0) {
      Some(variable) => {
        let value = evaluate_expression(value, &mut scope)?;

        if variable.constant {
          error(EvaluationErrorKind::ConstantReassignment(name.0), span)
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
      None => error(EvaluationErrorKind::UndefinedVariable(name.0), span),
    },
  }
}

fn evaluate_expression(
  expression: Expression,
  mut scope: &mut Scope,
) -> Result<Value, EvaluationError> {
  let Expression { kind, span } = expression;

  match kind {
    ExpressionKind::Void => Ok(Value::Void),
    ExpressionKind::Identifier(identifier) => match scope.get(&identifier.0) {
      Some(variable) => Ok(variable.value.clone()),
      None => error(EvaluationErrorKind::UndefinedVariable(identifier.0), span),
    },
    ExpressionKind::NumberLiteral(number) => Ok(Value::Number(number)),
    ExpressionKind::StringLiteral(string) => Ok(Value::String(string)),
    ExpressionKind::BooleanLiteral(boolean) => Ok(Value::Boolean(boolean)),
    ExpressionKind::ArrayLiteral(expressions) => {
      let mut array = Vec::new();
      for expression in expressions {
        array.push(evaluate_expression(expression, &mut scope)?);
      }
      Ok(Value::Array(array))
    }
    ExpressionKind::Index { iterable, index } => {
      let iterable = evaluate_expression(*iterable, &mut scope)?;
      let index = evaluate_expression(*index, &mut scope)?;
      match (iterable.clone(), index) {
        (Value::Array(array), Value::Number(number)) => {
          let index = number as usize;
          match array.get(index) {
            Some(value) => Ok(value.clone()),
            None => error(
              EvaluationErrorKind::IndexOutOfBounds(index, array.len()),
              span,
            ),
          }
        }
        (Value::String(string), Value::Number(number)) => {
          let index = number as usize;
          match string.chars().nth(index) {
            Some(character) => Ok(Value::String(character.to_string())),
            None => error(
              EvaluationErrorKind::IndexOutOfBounds(index, string.len()),
              span,
            ),
          }
        }
        _ => error(
          EvaluationErrorKind::InvalidType(
            iterable.as_ref().to_string(),
            vec!["Array".to_string()],
          ),
          span,
        ),
      }
    }
    ExpressionKind::Slice {
      iterable,
      start,
      end,
    } => {
      let iterable = evaluate_expression(*iterable, &mut scope)?;
      let array = match iterable.clone() {
        Value::Array(array) => array,
        Value::String(string) => string
          .chars()
          .map(|c| Value::String(c.to_string()))
          .collect(), // TODO: More efficient way to do this?
        _ => {
          return error(
            EvaluationErrorKind::InvalidType(
              iterable.as_ref().to_string(),
              vec!["Array".to_string(), "String".to_string()],
            ),
            span,
          );
        }
      };

      let start = match *start {
        Some(start) => match evaluate_expression(start, &mut scope)? {
          Value::Number(number) => number as usize,
          x => {
            return error(
              EvaluationErrorKind::InvalidType(x.as_ref().to_string(), vec!["Number".to_string()]),
              span,
            );
          }
        },
        None => 0,
      };

      let end = match *end {
        Some(end) => match evaluate_expression(end, &mut scope)? {
          Value::Number(number) => number as usize,
          x => {
            return error(
              EvaluationErrorKind::InvalidType(x.as_ref().to_string(), vec!["Number".to_string()]),
              span,
            );
          }
        },
        None => array.len(),
      };

      if start > end {
        return error(
          EvaluationErrorKind::InvalidRange(start as f64, end as f64),
          span,
        );
      }

      if end > array.len() {
        return error(
          EvaluationErrorKind::IndexOutOfBounds(end, array.len()),
          span,
        );
      }

      let mut result = Vec::new();
      for i in start..end {
        result.push(array[i].clone());
      }

      match iterable {
        Value::Array(_) => Ok(Value::Array(result)),
        Value::String(_) => Ok(Value::String(
          result
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(""),
        )),
        _ => unreachable!(),
      }
    }
    ExpressionKind::FunctionCall { name, arguments } => {
      let function = match scope.get(&name.0) {
        Some(variable) => variable.value.clone(),
        None => return error(EvaluationErrorKind::UndefinedVariable(name.0), span),
      };

      match function {
        Value::RustFunction {
          parameter_count,
          function,
        } => {
          if arguments.len() != parameter_count {
            return error(
              EvaluationErrorKind::IncorrectParameterCount(arguments.len(), parameter_count),
              span,
            );
          }
          let mut passed_parameters: Vec<Value> = Vec::new();
          for parameter in arguments {
            passed_parameters.push(evaluate_expression(parameter, &mut scope)?);
          }
          function(span, passed_parameters)
        }
        Value::Function { parameters, body } => {
          if parameters.len() != parameters.len() {
            return error(
              EvaluationErrorKind::IncorrectParameterCount(parameters.len(), parameters.len()),
              span,
            );
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
        _ => error(
          EvaluationErrorKind::InvalidType(
            function.as_ref().to_string(),
            vec!["Function".to_string()],
          ),
          span,
        ),
      }
    }
    ExpressionKind::Unary { operator, operand } => {
      let operand = evaluate_expression(*operand, &mut scope)?;
      match (operator.clone(), operand.clone()) {
        (UnaryOperator::Negate, Value::Number(number)) => Ok(Value::Number(-number)),
        (UnaryOperator::Not, Value::Boolean(boolean)) => Ok(Value::Boolean(!boolean)),
        _ => error(
          EvaluationErrorKind::InvalidOperator(
            operator.to_string(),
            operand.as_ref().to_string(),
            "".to_string(),
          ),
          span,
        ),
      }
    }
    ExpressionKind::Binary {
      operator,
      left,
      right,
    } => {
      match operator {
        // Make sure logical operators are short-circuited
        BinaryOperator::And => {
          let left = evaluate_expression(*left.clone(), &mut scope)?;
          match left {
            Value::Boolean(left_result) => {
              if !left_result {
                return Ok(Value::Boolean(false));
              }

              let right = evaluate_expression(*right.clone(), &mut scope)?;
              match right {
                Value::Boolean(right_result) => {
                  return Ok(Value::Boolean(right_result));
                }
                _ => {
                  return error(
                    EvaluationErrorKind::InvalidType(
                      right.as_ref().to_string(),
                      vec!["Boolean".to_string()],
                    ),
                    span,
                  );
                }
              }
            }
            _ => {
              return error(
                EvaluationErrorKind::InvalidType(
                  left.as_ref().to_string(),
                  vec!["Boolean".to_string()],
                ),
                span,
              );
            }
          }
        }
        BinaryOperator::Or => {
          let left = evaluate_expression(*left.clone(), &mut scope)?;
          match left {
            Value::Boolean(left_result) => {
              if left_result {
                return Ok(Value::Boolean(true));
              }

              let right = evaluate_expression(*right.clone(), &mut scope)?;
              match right {
                Value::Boolean(right_result) => {
                  return Ok(Value::Boolean(right_result));
                }
                _ => {
                  return error(
                    EvaluationErrorKind::InvalidType(
                      right.as_ref().to_string(),
                      vec!["Boolean".to_string()],
                    ),
                    span,
                  );
                }
              }
            }
            _ => {
              return error(
                EvaluationErrorKind::InvalidType(
                  left.as_ref().to_string(),
                  vec!["Boolean".to_string()],
                ),
                span,
              );
            }
          }
        }
        _ => {
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
                return error(EvaluationErrorKind::InvalidRange(left, right), span);
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

            // string, any type
            (BinaryOperator::Add, Value::String(left), right) => {
              Ok(Value::String(format!("{}{}", left, right.to_string())))
            }
            (BinaryOperator::Add, left, Value::String(right)) => {
              Ok(Value::String(format!("{}{}", left.to_string(), right)))
            }

            // unhandled cases
            (operator, left, right) => error(
              EvaluationErrorKind::InvalidOperator(
                operator.to_string(),
                left.as_ref().to_string(),
                right.as_ref().to_string(),
              ),
              span,
            ),
          }
        }
      }
    }
    ExpressionKind::If {
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
        _ => error(
          EvaluationErrorKind::InvalidType(
            condition.as_ref().to_string(),
            vec!["Boolean".to_string()],
          ),
          span,
        ),
      }
    }
    ExpressionKind::For {
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
        Value::String(string) => {
          let mut value = Vec::new();
          for character in string.chars() {
            scope.push_scope();
            scope.insert(
              variable.0.clone(),
              Variable {
                value: Value::String(character.to_string()),
                constant: true,
              },
            );
            value.push(evaluate_statement(*body.clone(), &mut scope)?);
            scope.pop_scope();
          }
          Ok(Value::Array(value))
        }
        _ => error(
          EvaluationErrorKind::InvalidType(
            iterable.as_ref().to_string(),
            vec!["Array".to_string()],
          ),
          span,
        ),
      }
    }
    ExpressionKind::While { condition, body } => {
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
            return error(
              EvaluationErrorKind::InvalidType(
                condition.as_ref().to_string(),
                vec!["Boolean".to_string()],
              ),
              span,
            );
          }
        }
      }

      Ok(Value::Array(value))
    }
  }
}
