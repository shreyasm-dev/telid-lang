print('Enter an operator (+, -, *, /, %): ');
let operator = readln();

while !(|| || || || (== operator '+') (== operator '-') (== operator '*') (== operator '/') (== operator '%')) {
  println('Invalid operator!');
  print('Enter an operator (+, -, *, /): ');
  operator = readln();
}

let fn read_number = {
  print('Enter a number: ');
  let num = parse(readln());
  while == num void {
    println('Invalid number!');
    print('Enter a number: ');
    num = parse(readln());
  }
  num;
}

let num1 = read_number();
let num2 = read_number();

print('Result: ');

if == operator '+'
  println(+ num1 num2);
else if == operator '-'
  println(- num1 num2);
else if == operator '*'
  println(* num1 num2);
else if == operator '/'
  println(/ num1 num2);
else if == operator '%'
  println(% num1 num2);
