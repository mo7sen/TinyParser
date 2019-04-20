use std::fs; 
use std::i64;

#[derive(Copy, Clone, Debug)]
enum State {
    START,
    INCOMMENT,
    INNUM,
    INID,
    INASSIGN,
}

enum TokenType {
    RESERVED,
    IDENTIFIER,
    NUMBER,
    COMMENT,
    SYMBOL,
}

#[derive(Debug)]
pub enum Token {
    RESERVED(String),
    IDENTIFIER(String),
    NUMBER(i64),
    COMMENT(String),
    SYMBOL(String),
}

pub fn tokenize<'a>(file_name: &'a str) -> Vec<Token> {
    let file_contents = read_file(file_name);
    let mut char_iter = file_contents.chars();
    let mut curr_char = char_iter.next();
    let mut curr_state = State::START;
    let mut tokens: Vec<Token> = Vec::new(); 
    let mut curr_val = String::new();

    while curr_char.is_some() {
        let c = curr_char.unwrap();
        match curr_state {
            State::START=> {
               if c.is_ascii_whitespace(){
                   curr_char = char_iter.next();
               } else if c == '{' {
                    curr_state = State::INCOMMENT;
                    curr_char = char_iter.next();
		    add_token("{", TokenType::SYMBOL, &mut tokens);
               } else if c.is_alphabetic() {
                    curr_state = State::INID;
                    curr_val += &c.to_string();
                    curr_char = char_iter.next();
               } else if c.is_digit(10) {
                    curr_state = State::INNUM;
                    curr_val += &c.to_string();
                    curr_char = char_iter.next();
               } else if c == ':' {
                    curr_state = State::INASSIGN;
                    curr_val += &c.to_string();
                    curr_char = char_iter.next();
               } else {
                    add_token(&c.to_string(), TokenType::SYMBOL, &mut tokens);
                    curr_char = char_iter.next();
               }
            },
            State::INCOMMENT=> {
               if c == '}' {
                    add_token(&curr_val.clone(), TokenType::COMMENT, &mut tokens);
                    add_token("}", TokenType::SYMBOL, &mut tokens);
                    curr_val.clear();
                    curr_state = State::START;
                    curr_char = char_iter.next();
               } else {
                    curr_val += &c.to_string();
                    curr_char = char_iter.next();
                    curr_state = State::INCOMMENT;
               }
            },
            State::INNUM=> {
               if c.is_digit(10) {
                   curr_val += &c.to_string();
                   curr_char = char_iter.next();
               } else {
                   add_token(&curr_val.clone(), TokenType::NUMBER, &mut tokens);
                   curr_val.clear();
                   curr_state = State::START;
               }
            },
            State::INID=> {
                if c.is_alphabetic() {
                    curr_val += &c.to_string();
                    curr_char = char_iter.next();
                } else {
                    add_token(&curr_val.clone(), get_token_type(&curr_val), &mut tokens);
                    curr_val.clear();
                    curr_state = State::START;
                }
            },
            State::INASSIGN=> {
                if c == '=' {
                    curr_val += &c.to_string();
                    add_token(&curr_val.clone(), TokenType::SYMBOL, &mut tokens);
                    curr_val.clear();
                    curr_char = char_iter.next();
                    curr_state = State::START;
                } else {
                    add_token(&curr_val.clone(), TokenType::SYMBOL, &mut tokens);
                    curr_val.clear();
                    curr_state = State::START;
                }
            },
        }
    }
    tokens
}

use std::path::Path;
use std::env::current_exe;

fn read_file<'a>(file_name: &'a str) -> String {
    let err_mess: String = String::from("Trouble reading the file");
    let mut path = current_exe().unwrap();
    path.pop();
    path.push(Path::new(file_name));
    fs::read_to_string(file_name)
        .expect(err_mess.as_str())
}

fn get_token_type(id: &str) -> TokenType {
    match id {
        "if"    |
        "then"  |
        "else"  |
        "repeat"|
        "end"   |
        "until" |
        "read"  |
        "write" => TokenType::RESERVED,
        _       => TokenType::IDENTIFIER,
    }
}

fn add_token<'a>(val: &'a str, tok_type: TokenType, tokens: &mut Vec<Token>) {
    match tok_type {
        TokenType::RESERVED => {tokens.push(Token::RESERVED(String::from(val)));},
        TokenType::NUMBER => {tokens.push(Token::NUMBER(i64::from_str_radix(val, 10).unwrap()));},
        TokenType::SYMBOL => {tokens.push(Token::SYMBOL(String::from(val)));},
        TokenType::IDENTIFIER => {tokens.push(Token::IDENTIFIER(String::from(val)));},
        TokenType::COMMENT => {tokens.push(Token::COMMENT(String::from(val)));},
    }
}
