# Telid

Telid is a dynamically-typed, interpreted language

This is a learning project. It's not good. At all.

If I were to go back and do things better, I would:

- Implement infix notation
- Use better index access syntax (the only reason it's a prefix operator is because it didn't work as postfix and I didn't want to spend time figuring out why)
- Figure out the semicolon mess

## Example

```rust
// this is a comment
/*
  this is a multiline comment
*/

let a = 1; // variables are mutable by default
let const b = 2; // unless you use the const keyword

let fn factorial n =
  if == n 0
    1
  else
    * n factorial(- n 1);

println(factorial(5)); // 120

let y = [1, 2, 3]; // this is an array literal

for i in y {
  println(+ + i ' - ' [i]y); // Index access is a prefix operator
}

let counter = 0;
while <= counter 10 {
  counter = + counter 1;
}
println(counter); // 11

println(* 2 3); // telid uses prefix notation

println(.. 1 10); // .. is the range operator (inclusive, inclusive)
// if you pass a non-integer to .., it will be converted to an integer through truncation

/*

Note: Semicolons are optional, but recommended

Take the following example:

let x = y

(* 2 3)

x would be equal to y(* 2 3), not y, as a first glance might suggest

let x = y;

(* 2 3)

Adding a semicolon would fix this

*/
```

Look at `examples/` for more examples
