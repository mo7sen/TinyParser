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
    RESERVED(Span),
    IDENTIFIER(Span),
    NUMBER(Span),
    COMMENT(Span),
    SYMBOL(Span),
}

impl Token {
    pub fn get_span(&self) -> Span{
        match self {
            Token::RESERVED(span) |
            Token::IDENTIFIER(span) |
            Token::NUMBER(span) |
            Token::COMMENT(span) |
            Token::SYMBOL(span) => {
                *span
            },
        }
    }
}

pub type Span = (usize, usize);

pub fn tokenize<'a>(file_contents: &'a str) -> Vec<Token> {

    //Need a hotfix for the ' '
    let mut char_iter = file_contents.chars();
    let mut curr_char = char_iter.next();
    let mut curr_state = State::START;
    let mut tokens: Vec<Token> = Vec::new();
    let (mut curr_index, mut curr_offset) = (0,1);

    while curr_char.is_some() {
        let c = curr_char.unwrap();
        match curr_state {
            State::START=> {
               if c.is_ascii_whitespace(){
                   curr_char = char_iter.next();
                   curr_index += c.len_utf8();
                   curr_offset += c.len_utf8();
               } else if c == '{' {
                    curr_state = State::INCOMMENT;
                    curr_char = char_iter.next();
		            add_token((curr_index, curr_offset), TokenType::SYMBOL, &mut tokens);
                    curr_index += c.len_utf8();
               } else if c.is_alphabetic() {
                    curr_state = State::INID;
                    curr_char = char_iter.next();
               } else if c.is_digit(10) {
                    curr_state = State::INNUM;
                    curr_char = char_iter.next();
               } else if c == ':' {
                    curr_state = State::INASSIGN;
                    curr_char = char_iter.next();
               } else {
                    add_token((curr_index, curr_offset), TokenType::SYMBOL, &mut tokens);
                    curr_index = curr_offset;
                    curr_offset += c.len_utf8();
                    curr_char = char_iter.next();
               }
            },
            State::INCOMMENT=> {
               if c == '}' {
                    add_token((curr_index, curr_offset), TokenType::COMMENT, &mut tokens);
                    curr_index = curr_offset;
                    curr_offset += c.len_utf8();
                    add_token((curr_index, curr_offset), TokenType::SYMBOL, &mut tokens);

                    curr_state = State::START;
                    curr_char = char_iter.next();
                    curr_index = curr_offset;
                    curr_offset += c.len_utf8();
               } else {
                    curr_offset += c.len_utf8();
                    curr_char = char_iter.next();
                    curr_state = State::INCOMMENT;
               }
            },
            State::INNUM=> {
               if c.is_digit(10) {
                   curr_offset += c.len_utf8();
                   curr_char = char_iter.next();
               } else {
                   add_token((curr_index, curr_offset), TokenType::NUMBER, &mut tokens);
                   curr_index = curr_offset;
                   curr_offset += c.len_utf8();
                   curr_state = State::START;
               }
            },
            State::INID=> {
                if c.is_alphabetic() {
                    curr_offset += c.len_utf8();
                    curr_char = char_iter.next();
                } else {
                    add_token((curr_index, curr_offset), get_token_type((curr_index, curr_offset), file_contents), &mut tokens);
                    curr_index = curr_offset;
                    curr_offset += c.len_utf8();
                    curr_state = State::START;
                }
            },
            State::INASSIGN=> {
                if c == '=' {
                    curr_offset += c.len_utf8();
                    add_token((curr_index, curr_offset), TokenType::SYMBOL, &mut tokens);

                    curr_index = curr_offset;
                    curr_offset += c.len_utf8();
                    curr_char = char_iter.next();
                    curr_state = State::START;
                } else {
                    add_token((curr_index, curr_offset), TokenType::SYMBOL, &mut tokens);
                    curr_index = curr_offset;
                    curr_offset += c.len_utf8();
                    curr_state = State::START;
                }
            },
        }
    }
    tokens
}

fn get_token_type(span: Span, src: &str) -> TokenType {
    match &src[span.0..span.1] {
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

fn add_token<'a>(val: Span, tok_type: TokenType, tokens: &mut Vec<Token>) {
    match tok_type {
        TokenType::RESERVED => {tokens.push(Token::RESERVED(val));},
        TokenType::NUMBER => {tokens.push(Token::NUMBER(val));},
        TokenType::SYMBOL => {tokens.push(Token::SYMBOL(val));},
        TokenType::IDENTIFIER => {tokens.push(Token::IDENTIFIER(val));},
        TokenType::COMMENT => {tokens.push(Token::COMMENT(val));},
    }
}
