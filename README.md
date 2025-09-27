# calc-cli

A simple command-line calculator written in Rust.  
Supports basic arithmetic expressions with operator precedence and parentheses.

## Features

- Addition, subtraction, multiplication, division, and exponentiation
- Parentheses for grouping
- Floating-point numbers
- Error handling for invalid input and division by zero

## Usage

1. **Build the project:**
   ```sh
   cargo build --release
   ```

2. **Run the calculator:**
   ```sh
   cargo run
   ```

3. **Type expressions at the prompt:**
   ```
   > 2 + 3 * 4
   14
   > (2 + 3) * 4
   20
   > 2 ^ 3
   8
   > quit
   ```

## Example

```
Simple Calculator - Type expressions or 'quit' to exit
> 1 + 2 * 3
7
> (1 + 2) * 3
9
> quit
```

## Project Structure

- `src/main.rs` — Command-line interface
- `src/lib.rs` — Tokenizer, parser, and evaluator

## Running Tests

```sh
cargo test
```

## License

MIT
