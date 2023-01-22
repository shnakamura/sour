use clap::Parser;
use args::SourArgs;
use interpreter::Interpreter;

mod args;
mod interpreter;

pub fn main() {
   let args = SourArgs::parse();

   let interpreter = Interpreter {
        args: args
   };

   interpreter.interpret();
}