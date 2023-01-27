use std::fs;
use std::path::Path;
use clap::Parser;
use interpreter::Interpreter;

mod interpreter;

#[derive(Parser)]
#[command(author, version)]
pub struct SourArgs {
    /// The brainfuck content or path to brainfuck file.
    #[arg()]
    pub content: String,

    /// The memory buffer size.
    #[arg(short, long, default_value_t = 30000)]
    pub size: usize,

    /// Whether to output raw bytes or encode to ASCII.
    #[arg(short, long, default_value_t = false)]
    pub raw: bool
}

pub fn main() {
   let args = SourArgs::parse();
   let path = Path::new(&args.content);

   if path.exists() && path.is_file() {
      match fs::read_to_string(args.content) {
         Ok(content) => Interpreter::interpret(content, args.size, args.raw),
         Err(error) => panic!("Could not read the file {:?}", error)
      } 
   }
   else {
      Interpreter::interpret(args.content, args.size, args.raw);
   }
}