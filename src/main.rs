pub const BUFFER_SIZE: usize = 30000;

pub fn main() {
    let mut memory_buffer = [0u8; BUFFER_SIZE];
    let mut memory_pointer = 0;

    let path = std::env::args().nth(1).expect("No path was given.");
    let file = std::fs::read_to_string(path).expect("Could not read the file from the given path.");

    interpret_file(&file, &mut memory_buffer, &mut memory_pointer);
}

pub fn interpret_file(file: &str, buffer: &mut [u8], pointer: &mut usize) {
    let mut char_pointer = 0;

    while char_pointer < file.len() {
        let char = file.chars().nth(char_pointer).unwrap();

        match char {
            '>' => {
                if *pointer < BUFFER_SIZE {
                    *pointer += 1;
                }
            },
            '<' => {
                if *pointer > 0 {
                    *pointer -= 1;
                }
            },
            '+' => {
                if buffer[*pointer] < u8::MAX {
                    buffer[*pointer] += 1;
                }
            },
            '-' => {
                if buffer[*pointer] > 0 {
                    buffer[*pointer] -= 1;
                }
            },
            '.' => {
                print!("{}", buffer[*pointer] as char);
            },
            ',' => {
                let mut input_text = String::new();
                std::io::stdin().read_line(&mut input_text).unwrap();
                buffer[*pointer] = input_text.trim().parse::<u8>().unwrap();
            },
            '[' => {
                if buffer[*pointer] == 0 {
                    let mut loops = 1;

                    while loops > 0 {
                        char_pointer += 1;

                        match file.chars().nth(char_pointer).unwrap() {
                            '[' => loops += 1,
                            ']' => loops -= 1,
                            _ => {}
                        }
                    }
                }
            },
            ']' => {
                if buffer[*pointer] != 0 {
                    let mut loops = 1;

                    while loops > 0 {
                        char_pointer -= 1;

                        match file.chars().nth(char_pointer).unwrap() {
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