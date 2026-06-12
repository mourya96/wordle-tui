# wordle-tui

A terminal-based Wordle clone built in Rust using [Ratatui](https://ratatui.rs).

## About

Guess a hidden 5-letter word in 6 attempts. After each guess, each tile changes color to show how close your guess was:

- Green: correct letter in the correct position
- Yellow: correct letter in the wrong position
- Gray: letter not in the word

## Installation

```bash
git clone https://github.com/mourya96/wordle-tui.git
cd wordle-tui
cargo build --release
```

## Usage

```bash
cargo run --release
```

### Controls

| Key       | Action                   |
|-----------|--------------------------|
| A-Z       | Type a letter            |
| Backspace | Delete the last letter   |
| Enter     | Submit the current guess |
| Esc       | Quit                     |

Guesses must be valid 5-letter words. The answer is chosen randomly at the start of each game.

## Dependencies

- [ratatui](https://github.com/ratatui/ratatui) — terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — cross-platform terminal input/output
- [rand](https://github.com/rust-random/rand) — For random word selection
