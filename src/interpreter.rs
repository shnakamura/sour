use std::path::Path;
use std::fs;

use crate::SourArgs;

pub struct Interpreter {
    pub args: SourArgs
}

impl Interpreter {
    pub fn interpret(&self) {
        let mut memory_buffer: Vec<u8> = vec![0u8; self.args.size];
        let mut memory_pointer = 0;

        let mut brainfuck_content = self.args.content.clone();
   
        if Path::new(&brainfuck_content).exists() {
            brainfuck_content = fs::read_to_string(&brainfuck_content).unwrap();
         }

        let mut char_pointer = 0;

        while char_pointer < brainfuck_content.len() {
            let char = brainfuck_content.chars().nth(char_pointer).unwrap();

            match char {
                '>' => {
                    if memory_pointer < self.args.size {
                        memory_pointer += 1;
                    }
                },
                '<' => {
                    if memory_pointer > 0 {
                        memory_pointer -= 1;
                    }
                },
                '+' => {
                    if memory_buffer[memory_pointer] < u8::MAX {
                        memory_buffer[memory_pointer] += 1;
                    }
                },
                '-' => {
                    if memory_buffer[memory_pointer] > 0 {
                        memory_buffer[memory_pointer] -= 1;
                    }
                },
                '.' => {
                    if self.args.raw {
                        print!("{}", memory_buffer[memory_pointer]);
                    }
                    else {
                        print!("{}", memory_buffer[memory_pointer] as char);
                    }
                },
                ',' => {
                    let mut input_text = String::new();
                    std::io::stdin().read_line(&mut input_text).unwrap();
                    memory_buffer[memory_pointer] = input_text.trim().parse::<u8>().unwrap();
                },
                '[' => {
                    if memory_buffer[memory_pointer] == 0 {
                        let mut loops = 1;

                        while loops > 0 {
                            char_pointer += 1;

                            match brainfuck_content.chars().nth(char_pointer).unwrap() {
                                '[' => loops += 1,
                                ']' => loops -= 1,
                                _ => {}
                            }
                        }
                    }
                },
                ']' => {
                    if memory_buffer[memory_pointer] != 0 {
                        let mut loops = 1;

                        while loops > 0 {
                            char_pointer -= 1;

                            match brainfuck_content.chars().nth(char_pointer).unwrap() {
                                '[' => loops -= 1,
                                ']' => loops += 1,
                                _ => {}
                            }
                        }
                    }
                },
                _ => { }
            }

            char_pointer += 1;
        }
    }
}