let fn fizzbuzz n =
  if == % n 15 0
    'FizzBuzz'
  else if == % n 3 0
    'Fizz'
  else if == % n 5 0
    'Buzz'
  else
    n;

for i in .. 1 15 println(fizzbuzz(i));

let count = 15;
while < count 30 {
  count = + count 1;
  println(fizzbuzz(count));
}
