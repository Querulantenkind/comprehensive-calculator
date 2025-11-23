# Comprehensive Terminal Calculator

A robust, feature-rich terminal-based calculator built in Rust using `ratatui` for the TUI and `evalexpr` for mathematical evaluation.

## Features

-   **Full TUI Interface**: Clean split-view interface with scrollable history and input.
-   **Math Evaluation**: Supports complex expressions, functions, and precedence.
-   **Variables**: Assign and use variables (e.g., `x = 5`, `x * 2`).
-   **Persistent History**: Calculations are saved between sessions.
-   **Clipboard Support**: Copy results directly to your system clipboard.
-   **Command Mode**: Built-in commands for managing the application state.
-   **Help System**: Integrated help popup with keybindings.

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

| Key | Action |
| --- | --- |
| `Enter` | Evaluate expression / Run command |
| `Up` / `Down` | Scroll through history |
| `Ctrl+y` | Copy selected result to clipboard |
| `?` / `F1` | Toggle help popup |
| `Esc` | Close popup / Quit application |

### Commands

Commands start with a colon (`:`):

-   `:clear`, `:c` - Clear history and variables.
-   `:help`, `:h` - Show the help popup.
-   `:quit`, `:q` - Save state and exit.

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
