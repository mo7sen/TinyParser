use std::env;


mod lexer;



fn main() {
    let args: Vec<String> = env::args().collect();
    let rel_path = &args[1];
    let tokens = lexer::tokenize(rel_path);
    print!("{:#?}", tokens);
}
