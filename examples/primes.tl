// Not efficient at all
// For a number n, it has to check if n is divisible by all numbers from 2 to n - 1

print('Enter a number > ');
let numbers = .. 1 parse(readln());

let fn f x = {
  if && (!= x 1) (!= x 2) {
    let result = true;

    for n in .. 2 (- x 1) {
      let is_not_divisible = != (% x n) 0;
      result = && result is_not_divisible;
    }

    result
  } else {
    true
  }
};

for n in numbers println(+ + n ' - ' if != n 1 if f(n) 'prime' else 'not prime' else 'not prime');
