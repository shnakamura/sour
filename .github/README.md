# 🚀 Sour
A customizable command line brainfuck interpreter.

## Installation
Building with [Cargo](https://github.com/rust-lang/cargo/):
```sh
$ git clone https://github.com/naakaamura/sour.git
$ cd sour
$ cargo build --release
```

Installing from [Crates.io](https://crates.io):
```sh
$ cargo install sour
```
## Usage
```
Usage: sour [OPTIONS] <CONTENT>

Arguments:
  <CONTENT>  The brainfuck content or path to brainfuck file

Options:
  -s, --size <SIZE>  The memory buffer size [default: 30000]
  -r, --raw          Whether to output raw bytes or encode to ASCII
  -h, --help         Print help
  -V, --version      Print version
```

## Examples

Interpreting from a file
```sh
$ sour hello-world.bf
# OUTPUT: Hello, world!
```

Interpreting raw byte values from a file
```sh
$ sour hello-world.bf -r
# OUTPUT: 72 101 108 108 111 44 32 87 111 114 108 100 33
```

Interpreting from raw brainfuck content
```sh
$ sour +++++.
# OUTPUT: ♣
```

Interpreting with a custom memory buffer size
```sh
$ sour <CONTENT> -s 69420
```
