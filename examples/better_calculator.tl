println('This calculator supports +, -, *, and /');
print('Enter an expression to evaluate (prefix notation): ');
let source = readln();

println('');

let fn lex source = {
  let i = 0;
  filter(while < i len(source) {
    let result = if != [i]source ' ' {
      if || (|| (|| (== [i]source '+') (== [i]source '-')) (== [i]source '*')) (== [i]source '/') {
        [i]source
      } else if != parse([i]source) void {
        let number = [i]source;
        while && (< (+ i 1) len(source)) (!= parse([+ i 1]source) void) {
          number = + number [+ i 1]source;
          i = + i 1;
        }
        parse(number)
      } else if || (== [i]source '(') (== [i]source ')') {
        [i]source
      } else {
        println(+ + 'Invalid character - \'' [i]source '\'');
        exit(1);
      }
    };

    i = + i 1;
    result
  }, "Void")
};

let tokens = lex(source);
println(+ 'Tokens: ' tokens);

// asts are like this:
// ['<op>', lhs, rhs] or ['number', value]
let fn parse tokens = {
  let tokens = tokens;
  let i = 0;
  let result = [];

  let fn parse_expr = {
    let tokens = tokens;
    let token = [i]tokens;

    i = + i 1;
    tokens = [i..]tokens;
    
    if == type(token) 'Number' {
      ['number', token, void] // Adding void to make it consistent with the other branches (so that the evaluation code can be just slightly simpler)
    } else if == token '(' {
      let expr = parse_expr();

      if != [i]tokens ')' {
        println('Expected \')\'');
        exit(1);
      }

      i = + i 1;
      tokens = [i..]tokens;
      expr
    } else if || (|| (|| (== token '+') (== token '-')) (== token '*')) (== token '/') {
      [token, parse_expr(), parse_expr()]
    } else {
      println(+ + 'Unexpected token - \'' token '\'');
      exit(1);
    }
  };
  
  parse_expr()
}

let ast = parse(tokens);
println(+ 'AST: ' ast);

let fn evaluate ast = {
  let ast = ast;
  let fn evaluate_ast ast = {
    let ast = ast;
    let op = [0]ast;
    let lhs = [1]ast;
    let rhs = [2]ast;

    if == op 'number' {
      lhs
    } else if == op '+' {
      + evaluate_ast(lhs) evaluate_ast(rhs)
    } else if == op '-' {
      - evaluate_ast(lhs) evaluate_ast(rhs)
    } else if == op '*' {
      * evaluate_ast(lhs) evaluate_ast(rhs)
    } else if == op '/' {
      / evaluate_ast(lhs) evaluate_ast(rhs)
    } else {
      println(+ + 'Unexpected operator - \'' op '\'');
      exit(1);
    }
  };

  evaluate_ast(ast)
};

let result = evaluate(ast);
println(+ 'Result: ' result);
