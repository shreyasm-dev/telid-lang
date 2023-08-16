#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
  Block(Vec<Statement>),
  LetStatement {
    name: Identifier,
    value: Expression,
    constant: bool,
  },
  ExpressionStatement(Expression),
  FunctionDeclaration {
    name: Identifier,
    parameters: Vec<Identifier>,
    body: Box<Statement>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
  Void,
  Identifier(Identifier),
  NumberLiteral(f64),
  StringLiteral(String),
  BooleanLiteral(bool),
  ArrayLiteral(Vec<Expression>),
  FunctionCall {
    name: Identifier,
    parameters: Vec<Identifier>,
  },
  If {
    condition: Box<Expression>,
    consequence: Box<Statement>,
    alternative: Option<Box<Statement>>,
  },
  For {
    variable: Identifier,
    iterable: Box<Expression>,
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
  Negate,
  Not,
}
