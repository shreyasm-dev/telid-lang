# slash

slash is a dynamically-typed, interpreted scripting language

## Example

```rust
// this is a comment
/*
  this is a multiline comment
*/

let a = 1 // variables are mutable by default
let const b = 2 // unless you use the const keyword

let fn factorial n = // functions are first-class
  if n == 0
    1
  else
    n * factorial(n - 1)

println(factorial(5)) // 120
println factorial 5 // parentheses are optional

println(|/some/path|) // |/some/path| is a path literal
// the pipe characters make sure that the path is oarsed correctly
// it gets converted to a string anyway, so they're optional

let y = [1, 2, 3] // this is an array literal

for i in y {
  println('\(i) - \(y[i])') // use \() for string interpolation (like in swift)
}
```
