print('Enter an operator (+, -, *, /): ');
let operator = readln();

while !(|| || || (== operator '+') (== operator '-') (== operator '*') (== operator '/')) {
  println('Invalid operator!');
  print('Enter an operator (+, -, *, /): ');
  operator = readln();
}

print('Enter a number: ');
let num1 = parse(readln());
while == num1 void {
  println('Invalid number!');
  print('Enter a number: ');
  num1 = parse(readln());
}

print('Enter a number: ');
let num2 = parse(readln());
while == num2 void {
  println('Invalid number!');
  print('Enter a number: ');
  num2 = parse(readln());
}

print('Result: ');

if == operator '+'
  println(+ num1 num2);
else if == operator '-'
  println(- num1 num2);
else if == operator '*'
  println(* num1 num2);
else if == operator '/'
  println(/ num1 num2);
