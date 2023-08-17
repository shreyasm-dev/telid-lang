use crate::{
  evaluator::{
    evaluate,
    value::{Value, Variable},
    Scope,
  },
  lexer::Lexer,
  parser::parser,
};
use chumsky::Parser;

#[test]
fn test_expression() {
  let source = "let x = + 5 * 5 2;";

  let mut lexer = Lexer::new(source);
  let tokens = lexer.lex(false);
  let tokens = tokens.iter().map(|t| t.kind.clone()).collect::<Vec<_>>();

  let ast = parser().parse(tokens);
  let ast = ast.unwrap();

  let scope = Scope::new();
  let result = evaluate(ast, scope);
  let result = result.unwrap();

  assert_eq!(
    *result.get(&String::from("x")).unwrap(),
    Variable {
      value: Value::Number(15.0),
      constant: false,
    }
  );
}
