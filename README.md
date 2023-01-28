# 🚀 Sour
A customizable command line brainfuck interpreter.

## Installation
For manual installations, have a look at the [releases](https://github.com/naakaamura/sour/releases) page. 

With [Cargo](https://github.com/rust-lang/cargo/):

```
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
```
$ sour hello-world.bf
# OUTPUT: Hello, world!
```

Interpreting raw byte values from a file
```
$ sour hello-world.bf -r
# OUTPUT: 7210110810811144328711111410810033
```

Interpreting from raw brainfuck content
```
$ sour +++++.
# OUTPUT: ♣
```
