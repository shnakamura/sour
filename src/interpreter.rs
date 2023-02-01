pub struct Interpreter { }

impl Interpreter {
    pub fn interpret(content: String, size: usize, raw: bool) {
        let mut memory_buffer: Vec<u8> = vec![0u8; size];
        let mut memory_pointer = 0;
        let mut char_pointer = 0;

        while char_pointer < content.len() {
            if let Some(char) = content.chars().nth(char_pointer) {
                match char {
                    '>' => {
                        if memory_pointer < size {
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
                        if raw {
                            print!("{} ", memory_buffer[memory_pointer]);
                        }
                        else {
                            print!("{}", memory_buffer[memory_pointer] as char);
                        }
                    },
                    ',' => {
                        let mut input_text = String::new();

                        if let Err(error) = std::io::stdin().read_line(&mut input_text) {
                            println!("Could not read user input {:?}", error);
                        }

                        match input_text.trim().parse::<u8>() {
                            Ok(parsed) => memory_buffer[memory_pointer] = parsed,
                            Err(error) => println!("Could not parse the user input {:?}", error)
                        }
                    },
                    '[' => {
                        if memory_buffer[memory_pointer] == 0 {
                            let mut loops = 1;
    
                            while loops > 0 {
                                char_pointer += 1;
    
                                match char {
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
    
                                if let Some(char) = content.chars().nth(char_pointer) {
                                    match char {
                                        '[' => loops -= 1,
                                        ']' => loops += 1,
                                        _ => {}
                                    }
                                }
                            }
                        }
                    },
                    _ => { }
                }

                char_pointer += 1;
                continue;
            } 

            println!("Could not read the {}th char of the given content", char_pointer);
        }
    }
}