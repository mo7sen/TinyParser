use std::env;


use crate::lexer::tokenize;



fn main() {
    let args: Vec<String> = env::args().collect();
    let rel_path = &args[1];
    let tokens = tokenize(rel_path);
    print!("{:#?}", tokens);
}
