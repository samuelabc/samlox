# samlox
- Implementation of Lox interpreter from the book [Crafting Interpreters](https://craftinginterpreters.com).
- Rust implementation based on [tdp2110/crafting-interpreters-rs](https://github.com/tdp2110/crafting-interpreters-rs) project.
- Syntax of the Lox language can be found [here](https://craftinginterpreters.com/the-lox-language.html).

## Progress
- [x] Lexing
- [ ] ...

## Usage
- run the interpreter
```
cargo run -- ./test/fib.lo
```
- run the interpreter, and show all scanned tokens
```
cargo run -- --show-tokens ./test/fib.lo
```
