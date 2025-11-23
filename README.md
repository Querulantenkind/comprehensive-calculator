# Comprehensive Terminal Calculator

A robust, feature-rich terminal-based calculator built in Rust using `ratatui` for the TUI and `evalexpr` for mathematical evaluation.

## Features

-   **Full TUI Interface**: Clean split-view interface with history and input.
-   **Math Evaluation**: Supports complex expressions, functions, and precedence.
-   **Variables**: Assign and use variables (e.g., `x = 5`, `x * 2`).
-   **History Navigation**: Scroll through past calculations with Up/Down arrow keys.
-   **Error Highlighting**: Visual feedback for invalid expressions.

## Installation

Ensure you have [Rust and Cargo installed](https://rustup.rs/).

```bash
git clone https://github.com/yourusername/comprehensive-calculator.git
cd comprehensive-calculator
cargo build --release
```

## Usage

Run the calculator:

```bash
cargo run
```

### Controls

-   **Type**: Enter mathematical expressions.
-   **Enter**: Evaluate expression.
-   **Up / Down**: Scroll through history.
-   **Esc**: Quit the application.

### Examples

```text
> 2 + 2
4

> a = 10
10

> b = 5
5

> (a + b) * 2
30

> sin(0)
0
```

## License

MIT
