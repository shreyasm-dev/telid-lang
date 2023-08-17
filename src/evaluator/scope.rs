use super::value::{Value, Variable};
use scoped_stack::ScopedStack;

pub type Scope = ScopedStack<String, Variable>;

pub fn default() -> Scope {
  let mut scope = Scope::new();

  scope.insert(
    String::from("println"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 1,
        function: |parameters| {
          println!("{}", parameters[0].to_string());
          Ok(Value::Void)
        },
      },
      constant: true,
    },
  );

  scope.insert(
    String::from("print"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 1,
        function: |parameters| {
          print!("{}", parameters[0].to_string());
          Ok(Value::Void)
        },
      },
      constant: true,
    },
  );

  scope
}
