# Grep Implementation in Rust

This project is a simplified implementation of the `grep` command-line utility written in Rust. It parses regular expressions and searches for matching patterns in input text.

## Features

- Regular expression pattern matching
- Support for character classes and escape sequences
- Efficient character-by-character processing
- Standard Unix-like command-line interface

## Supported Pattern Syntax

The implementation currently supports:

- **Character Classes**: `[abc]` matches any character in the set, `[^abc]` matches any character not in the set
- **Escape Sequences**:
  - `\d` matches any digit
  - `\w` matches any alphanumeric character
  - `\s` matches any whitespace character
- **Literal Characters**: Regular characters match themselves

## Usage

```bash
echo -n "input text" | ./your_program.sh -E "pattern"
```

The program will exit with status code 0 if the pattern matches the input, and 1 if it doesn't.

### Examples

```bash
# Match a line containing a digit followed by " apple"
echo -n "1 apple" | ./your_program.sh -E "\d apple"

# Match a line containing 3 digits followed by " apples"
echo -n "100 apples" | ./your_program.sh -E "\d\d\d apples"

# Match a pattern with multiple character types
echo -n "3 dogs" | ./your_program.sh -E "\d \w\w\ws"
```

## Building

To build the project:

```bash
cargo build --release
```

The binary will be located at `target/release/your_program`.

## License

This project is open source and available under the MIT License.
