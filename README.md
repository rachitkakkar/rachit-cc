# rachit-cc
A toy compiler for a simplified imperative language (SIL) written Rust.

Here is an example of a SIL program that recursively 
calculates the Fibonacci numbers:
```
# Compute the x'th Fibonacci number.
def fib(x) {
  if (x < 3) {
    return 1
  }
  else {
    return fib(x-1)+fib(x-2)
  }
}

# This expression will compute the 40th number.
fib(40)
```

And here are all the supported keywords in SIL:
```def if else while return break continue int bool void true```


# Planned Features
- Handwritten table/switch driven lexer (also known as a *scanner* or *tokenizer*) ✅
- Handwritten Recursive Descent parser
- Code generation to ARM assembly
- Language support for floating point numbers, basic arithmetic operators, mutable variables, function definitions and calls, conditionals, and while loops.
- 100% unit test coverage

# More

# Sources
The syntax is loosely inspired by the Decaf language reference used in CS 432 and CS 630 at James Madison University.

"Crafting Interpreters" by Robert Nystrom