use std::fs;
use std::env;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    print!("{}",file_path);
    let file = fs::read_to_string(file_path)
        .expect("Cannot read file");

    let contents: Vec<char> = file.chars().collect(); 

    let mut tokenizer = lexer::Lexer::new(contents);
    let tokens = tokenizer.scan();
    print!("{:?}", tokens)
}
