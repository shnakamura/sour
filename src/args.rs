use clap::Parser;

#[derive(Parser)]
#[command(author, version)]
pub struct SourArgs {
    /// The brainfuck content or path to brainfuck file.
    #[arg(short, long)]
    pub content: String,

    /// The memory buffer size.
    #[arg(short, long, default_value_t = 30000)]
    pub size: usize,

    /// Whether to output raw bytes or encode to ASCII.
    #[arg(short, long, default_value_t = false)]
    pub raw: bool
}