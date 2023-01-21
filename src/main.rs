fn main() {
    let path = std::env::args().nth(1).expect("No path was given.");
    let file = std::fs::read_to_string(path).expect("Could not read the file from the given path.");

    interpret_file(&file);
}

fn interpret_file(file: &str) {
    let char = file.chars();

    for c in char {
        interpret_char(&c);
    }
}

fn interpret_char(char: &char) {
    match *char {
        _ => println!("Unknown instruction.")
    }
}