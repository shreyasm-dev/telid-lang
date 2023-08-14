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

println [/some/path] // [/some/path] is a path literal
// the brackets make sure that the path is oarsed correctly
// it gets converted to a string anyway, so they're optional

$cd [/some/path] // $ is used for shell commands
$cd([/some/path]) // shell commands are treated just like functions, so you can use parentheses if you want

${
  echo "this is a shell block"
  echo "you can do this: $a"
  echo "or this: ${b}"
  echo "or even this: ${factorial(5)}"
  echo "normal slash code is not parsed in shell blocks (or validated!)"
}

$[/bin/zsh]{
  echo "the text inside the brackets is the shell to use"
  echo "it's piped into the shell"
}
```
