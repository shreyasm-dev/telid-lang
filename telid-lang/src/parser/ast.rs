use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
  pub kind: StatementKind,
  pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
  Block(Vec<Statement>),
  Let {
    name: Identifier,
    value: Expression,
    constant: bool,
  },
  Expression(Expression),
  FunctionDeclaration {
    name: Identifier,
    parameters: Vec<Identifier>,
    body: Box<Statement>,
  },
  Assignment {
    name: Identifier,
    value: Expression,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
  pub kind: ExpressionKind,
  pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
  Void,
  Identifier(Identifier),
  NumberLiteral(f64),
  StringLiteral(String),
  BooleanLiteral(bool),
  ArrayLiteral(Vec<Expression>),
  Index {
    iterable: Box<Expression>,
    index: Box<Expression>,
  },
  FunctionCall {
    name: Identifier,
    arguments: Vec<Expression>,
  },
  If {
    condition: Box<Expression>,
    consequence: Box<Statement>,
    alternative: Box<Option<Statement>>,
  },
  For {
    variable: Identifier,
    iterable: Box<Expression>,
    body: Box<Statement>,
  },
  While {
    condition: Box<Expression>,
    body: Box<Statement>,
  },
  Binary {
    operator: BinaryOperator,
    left: Box<Expression>,
    right: Box<Expression>,
  },
  Unary {
    operator: UnaryOperator,
    operand: Box<Expression>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  Equal,
  NotEqual,
  LessThan,
  LessThanOrEqual,
  GreaterThan,
  GreaterThanOrEqual,
  And,
  Or,
  Range,
}

impl ToString for BinaryOperator {
  fn to_string(&self) -> String {
    match self {
      BinaryOperator::Add => "+".to_string(),
      BinaryOperator::Subtract => "-".to_string(),
      BinaryOperator::Multiply => "*".to_string(),
      BinaryOperator::Divide => "/".to_string(),
      BinaryOperator::Modulo => "%".to_string(),
      BinaryOperator::Equal => "==".to_string(),
      BinaryOperator::NotEqual => "!=".to_string(),
      BinaryOperator::LessThan => "<".to_string(),
      BinaryOperator::LessThanOrEqual => "<=".to_string(),
      BinaryOperator::GreaterThan => ">".to_string(),
      BinaryOperator::GreaterThanOrEqual => ">=".to_string(),
      BinaryOperator::And => "&&".to_string(),
      BinaryOperator::Or => "||".to_string(),
      BinaryOperator::Range => "..".to_string(),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
  Identity,
  Negate,
  Not,
}

impl ToString for UnaryOperator {
  fn to_string(&self) -> String {
    match self {
      UnaryOperator::Identity => "+".to_string(),
      UnaryOperator::Negate => "-".to_string(),
      UnaryOperator::Not => "!".to_string(),
    }
  }
}
