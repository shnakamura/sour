use clap::Parser;
use args::SourArgs;
use interpreter::Interpreter;

mod args;
mod interpreter;

pub fn main() {
   let interpreter = Interpreter {
        args: SourArgs::parse()
   };

   interpreter.interpret();
}