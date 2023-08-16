pub struct Program {
  pub statements: Vec<Statement>,
}

pub enum Statement {
  Block {
    statements: Vec<Statement>,
  },
  LetStatement {
    name: Identifier,
    value: Expression,
    constant: bool,
  },
  ExpressionStatement {
    expression: Expression,
  },
  FunctionDeclaration {
    name: Identifier,
    parameters: Vec<Identifier>,
    body: Box<Statement>,
  },
}

pub enum Expression {
  Void,
  Identifier(Identifier),
  NumberLiteral(f64),
  StringLiteral(&'static str),
  BooleanLiteral(bool),
  ArrayLiteral(Vec<Expression>),
  FunctionCall {
    parameters: Vec<Identifier>,
    body: Box<Statement>,
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

pub struct Identifier {
  pub value: &'static str,
}

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

pub enum UnaryOperator {
  Negate,
  Not,
}
