# slash

slash is a dynamically-typed, interpreted scripting language

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

println(* 2 3); // slash uses prefix notation

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
