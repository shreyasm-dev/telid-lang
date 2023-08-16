#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<Statement>);

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
  Block(Vec<Statement>),
  LetStatement {
    name: Identifier,
    value: Box<Statement>,
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
  ArrayLiteral(Vec<Statement>),
  FunctionCall {
    parameters: Vec<Identifier>,
    body: Box<Statement>,
  },
  If {
    condition: Box<Statement>,
    consequence: Box<Statement>,
    alternative: Option<Box<Statement>>,
  },
  For {
    variable: Identifier,
    iterable: Box<Statement>,
    body: Box<Statement>,
  },
  Binary {
    operator: BinaryOperator,
    left: Box<Statement>,
    right: Box<Statement>,
  },
  Unary {
    operator: UnaryOperator,
    operand: Box<Statement>,
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
