use std::cmp::{max, min};
use std::iter::Peekable;
use std::slice::Iter;
use std::usize;
use tiny_lexer::lexer::{tokenize, Span, Token};

#[derive(Debug, Clone)]
pub enum NodeType {
    Program,
    StmtSeq,
    Stmt(StmtType),
    Op(OpType),
    Term,
    Factor,
    Exp,
    SimplExp,
    Identifier,
    Number,
    Keyword,
    Error(ErrorType, String),
    Root,
    Null,
    Symbol,
    OpeningBrace,
    ClosingBrace,
}

#[derive(Debug, Clone)]
pub enum StmtType {
    IfStmt,
    RepeatStmt,
    WriteStmt,
    ReadStmt,
    AssignStmt,
    Illegal,
}

#[derive(Debug, Clone)]
pub enum OpType {
    MulOp,
    AddOp,
    CompOp,
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    IllegalStmt,
    UnexpectedEOF,
    MissingThenKeyword,
    MissingUntilKeyword,
    MissingAssignOp,
    MissingAddOp,
    MissingMulOp,
    MissingCmpOp,
    MissingClosingBracket,
    IllegalFactor,
    NonEndedIfStmt,
    ExpectedFactor,
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub span: Span,
    pub n_type: NodeType,
    pub value: &'a str,
    pub children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    fn new() -> Node<'a> {
        Node {
            span: (usize::MAX, 0),
            n_type: NodeType::Null,
            value: "",
            children: vec![],
        }
    }

    fn add_child(&mut self, child: Node<'a>) {
        self.span.0 = min(self.span.0, child.span.0);
        self.span.1 = max(self.span.1, child.span.1);
        self.children.push(child);
    }

    pub fn print_dbg(&self, prefix: String) {
        println!("{}{:?}: {}", prefix, self.n_type, self.value);
        for node in &self.children {
            node.print_dbg(String::from("\t") + prefix.as_str());
        }
    }

    fn get_content(&self, src: &'a str) -> &'a str {
        &src[self.span.0..self.span.1]
    }

    fn reduce(&self) -> Node<'a>{
        self.children.get(0).unwrap().clone()
    }
}

pub fn parse<'a>(src: &'a str, simplified: bool) -> Node {
    let tokens: Vec<Token> = tokenize(src, false);
    let mut token_iter = Box::new(tokens.iter()).peekable();

    let mut program_node = Node::new();
    program_node.n_type = NodeType::Program;
    stmt_seq(&mut token_iter, &mut program_node, src, simplified);
    program_node.value = program_node.get_content(src);

    program_node
}

fn stmt_seq<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    let mut stmt_seq_node = Node::new();
    stmt_seq_node.n_type = NodeType::StmtSeq;

    if simplified {
        stmt(token_iter, parent_node, src, simplified);
    } else {
        stmt(token_iter, &mut stmt_seq_node, src, simplified);
    }

    loop {
        if match_tok(token_iter.peek(), ";", src) {
            token_iter.next();
            if simplified {
                stmt(token_iter, parent_node, src, simplified);
            } else {
                stmt(token_iter, &mut stmt_seq_node, src, simplified);
            }
        } else {
            break;
        }
    }
    if !simplified {
        stmt_seq_node.value = stmt_seq_node.get_content(src);
        parent_node.add_child(stmt_seq_node);
    }
}

fn stmt<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    let mut stmt_node = Node::new();
    let mut err = false;
    if let Some(&token) = token_iter.peek() {
        let mut stmt_type = StmtType::Illegal;
        match get_tok_content(token, src) {
            "if" => {
                stmt_type = StmtType::IfStmt;
                if_stmt(token_iter, &mut stmt_node, src, simplified);
            }
            "repeat" => {
                stmt_type = StmtType::RepeatStmt;
                repeat_stmt(token_iter, &mut stmt_node, src, simplified);
            }
            "read" => {
                stmt_type = StmtType::ReadStmt;
                read_stmt(token_iter, &mut stmt_node, src, simplified);
            }
            "write" => {
                stmt_type = StmtType::WriteStmt;
                write_stmt(token_iter, &mut stmt_node, src, simplified);
            }
            _ => {
                if let Token::IDENTIFIER(_) = token {
                    stmt_type = StmtType::AssignStmt;
                    assign_stmt(token_iter, &mut stmt_node, src, simplified);
                } else {
                    err = true;
                    add_error(parent_node, ErrorType::IllegalStmt, "Illegal Statement Error:\nExpected one of {'if', 'read', 'write', 'repeat', Identifier}. Found: '".to_string() + get_tok_content(token, src) + "'\nSuggested Fix:\tCheck if you have a semicolon(';') after your last statement.")
                }
            }
        }
        if !err {
            stmt_node.n_type = NodeType::Stmt(stmt_type);
            stmt_node.value = stmt_node.get_content(src);
            parent_node.add_child(stmt_node);
        }
    } else {
        add_error(parent_node, ErrorType::IllegalStmt, "Illegal Statement Error:\nSuggested Fix:\tCheck if you have a semicolon(';') after your last statement.".to_string());
    }
}

fn if_stmt<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    if !simplified {
        let mut if_node = Node::new();
        if_node.n_type = NodeType::Keyword;
        if_node.span = token_iter.next().unwrap().get_span();
        if_node.value = if_node.get_content(src);
        parent_node.add_child(if_node);
    } else {
        token_iter.next();
    }

    exp(token_iter, parent_node, src, simplified);

    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "then", src) {
            if !simplified {
                let mut then_node = Node::new();
                then_node.n_type = NodeType::Keyword;
                then_node.span = token_iter.next().unwrap().get_span();
                then_node.value = then_node.get_content(src);
                parent_node.add_child(then_node);
            } else {
                token_iter.next();
            }
        } else {
            add_error(parent_node, ErrorType::MissingThenKeyword, "IllegalIfStatementSyntax:\nExpected 'then' after the `if exp ->...<- stmtseq end`. Found: '".to_string() + get_tok_content(token_iter.peek().unwrap(), src) + "'\nSuggested Fix:\tAdd the missing 'then' keyword.");
        }
    } else {
        add_error(parent_node, ErrorType::UnexpectedEOF, "Unexpected EOF:\nExpected 'then' after the `if exp ->...<- stmtseq end`. Found: Early EOF".to_string());
    }

    stmt_seq(token_iter, parent_node, src, simplified);

    //Optional Part starts here

    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "else", src) {
            if !simplified {
                let mut else_node = Node::new();
                else_node.n_type = NodeType::Keyword;
                else_node.span = token_iter.next().unwrap().get_span();
                else_node.value = else_node.get_content(src);
                parent_node.add_child(else_node);
            } else {
                token_iter.next();
            }

            stmt_seq(token_iter, parent_node, src, simplified);
        }
    }

    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "end", src) {
            if !simplified {
                let mut end_node = Node::new();
                end_node.n_type = NodeType::Keyword;
                end_node.span = token_iter.next().unwrap().get_span();
                end_node.value = end_node.get_content(src);
                parent_node.add_child(end_node);
            } else {
                token_iter.next();
            }
        } else {
            add_error(parent_node, ErrorType::NonEndedIfStmt, "NonEndedIfStmt: All If Statements should be ended with the 'end'.\n\tSuggested Fix:\tAdd 'end' in its respective place.".to_string());
        }
    } else {
        add_error(parent_node, ErrorType::UnexpectedEOF, "Unexpected EOF:\nExpected 'end' keyword to close the If Statement, Found: Early EOF\nSuggested Fix:\tAdd 'end' in its respective place.".to_string());
    }
}

fn repeat_stmt<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    if !simplified {
        let mut repeat_node = Node::new();
        repeat_node.n_type = NodeType::Keyword;
        repeat_node.span = token_iter.next().unwrap().get_span();
        repeat_node.value = repeat_node.get_content(src);
        parent_node.add_child(repeat_node);
    } else {
        token_iter.next();
    }

    stmt_seq(token_iter, parent_node, src, simplified);

    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "until", src) {
            if !simplified {
                let mut until_node = Node::new();
                until_node.n_type = NodeType::Keyword;
                until_node.span = token_iter.next().unwrap().get_span();
                until_node.value = until_node.get_content(src);
                parent_node.add_child(until_node);
            } else {
                token_iter.next();
            }
        } else {
            add_error(parent_node, ErrorType::MissingUntilKeyword, "IllegalRepeatStatementSyntax:\nExpected 'until' at `repeat stmtseq ->....<- exp`. Found: '".to_string() + get_tok_content(token_iter.peek().unwrap(), src) + "'\nSuggested Fix:\tAdd the missing 'until' keyword.");
        }
    } else {
        add_error(parent_node, ErrorType::UnexpectedEOF, "Unexpected EOF:\nExpected 'until' at `repeat stmtseq ->....<- exp, Found: Early EOF\nSuggested Fix:\tAdd 'until' at its respective place.".to_string());
    }

    exp(token_iter, parent_node, src, simplified);
}

fn assign_stmt<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    identifier(token_iter, parent_node, src);

    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), ":=", src) {
            if !simplified {
                let mut ass_node = Node::new();
                ass_node.n_type = NodeType::Symbol;
                ass_node.span = token_iter.next().unwrap().get_span();
                ass_node.value = ass_node.get_content(src);
                parent_node.add_child(ass_node);
            } else {
                token_iter.next();
            }
        } else {
            add_error(parent_node, ErrorType::MissingAssignOp, "IllegalAssignmentSyntax:\nExpected AssignmentOperator ':=' at `identifier ->....<- exp`. Found: '".to_string() + get_tok_content(token_iter.peek().unwrap(), src) + "'\nSuggested Fix:\tAdd the missing ':=' keyword.");
        }
    } else {
        add_error(parent_node, ErrorType::UnexpectedEOF, "Unexpected EOF:\nExpected AssignmentOperator ':=' at `identifier ->....<- exp`, Found: Early EOF\nSuggested Fix:\tRemove the dangling identifier at the end of the statement sequence.".to_string());
    }

    exp(token_iter, parent_node, src, simplified);
}

fn read_stmt<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    if !simplified {
        let mut read_node = Node::new();
        read_node.n_type = NodeType::Keyword;
        read_node.span = token_iter.next().unwrap().get_span();
        read_node.value = read_node.get_content(src);
        parent_node.add_child(read_node);
    } else {
        token_iter.next();
    }
    if let Some(token) = token_iter.peek(){
        if let Token::IDENTIFIER(_) = token{
            identifier(token_iter, parent_node, src);
        } else {
            //TODO: ExpectedIdentifier
        }
    } else {
        //TODO:UnexpectedEOF
    }
}

fn write_stmt<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    if !simplified {
        let mut write_node = Node::new();
        write_node.n_type = NodeType::Keyword;
        write_node.span = token_iter.next().unwrap().get_span();
        write_node.value = write_node.get_content(src);
        parent_node.add_child(write_node);
    } else {
        token_iter.next();
    }
    if let Some(_) = token_iter.peek(){
        exp(token_iter, parent_node, src, simplified);
    } else {
        //TODO: UnexpectedEOF: Expected expression after the 'write' keyword
    }
}

//==============Start Of: Ops==============
fn add_op<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "+", src) || match_tok(token_iter.peek(), "-", src) {
            if !simplified {
                let mut addop_node = Node::new();
                addop_node.n_type = NodeType::Op(OpType::AddOp);
                addop_node.span = token_iter.next().unwrap().get_span();
                addop_node.value = addop_node.get_content(src);
                parent_node.add_child(addop_node);
            } else {
                parent_node.n_type = NodeType::Op(OpType::AddOp);
                parent_node.span = token_iter.next().unwrap().get_span();
                parent_node.value = parent_node.get_content(src);
            }
        } else {
            //TODO: MissingAddOperator
        }
    } else {
        //TODO: UnexpectedEOF
    }
}

fn mulop<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "*", src) || match_tok(token_iter.peek(), "/", src) {
            if !simplified {
                let mut mulop_node = Node::new();
                mulop_node.n_type = NodeType::Op(OpType::MulOp);
                mulop_node.span = token_iter.next().unwrap().get_span();
                mulop_node.value = mulop_node.get_content(src);
                parent_node.add_child(mulop_node);
            } else {
                parent_node.n_type = NodeType::Op(OpType::MulOp);
                parent_node.span = token_iter.next().unwrap().get_span();
                parent_node.value = parent_node.get_content(src);
            }
        } else {
            //TODO: MissingMulOperator
        }
    } else {
        //TODO: UnexpectedEOF
    }
}

fn comp_op<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "<", src) || match_tok(token_iter.peek(), "=", src) {
            if !simplified {
                let mut compop_node = Node::new();
                compop_node.n_type = NodeType::Op(OpType::CompOp);
                compop_node.span = token_iter.next().unwrap().get_span();
                compop_node.value = compop_node.get_content(src);
                parent_node.add_child(compop_node);
            } else {
                parent_node.n_type = NodeType::Op(OpType::CompOp);
                parent_node.span = token_iter.next().unwrap().get_span();
                parent_node.value = parent_node.get_content(src);
            }
        } else {
            //TODO: MissingCompOperator
        }
    } else {
        //TODO: UnexpectedEOF
    }
}
//==============End Of: Ops==============

fn exp<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    let mut exp_node = Node::new();
    let mut opped = false;

    if !simplified {
        exp_node.n_type = NodeType::Exp;
    }
    simple_exp(token_iter, &mut exp_node, src, simplified);

    //Optional part starts here
    loop {
        if match_tok(token_iter.peek(), "<", src) || match_tok(token_iter.peek(), "=", src) {
            opped = true;
            comp_op(token_iter, &mut exp_node, src, simplified);
            simple_exp(token_iter, &mut exp_node, src, simplified);
        } else {
            break;
        }
    }

    if !simplified {
        exp_node.value = exp_node.get_content(src);
    }
    if !opped {
        parent_node.add_child(exp_node.reduce());
    } else {
        parent_node.add_child(exp_node);
    }
}

fn simple_exp<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    let mut sexp_node = Node::new();
    let mut opped = false;

    if !simplified{
        sexp_node.n_type = NodeType::SimplExp;
    }

    term(token_iter, &mut sexp_node, src, simplified);

    loop {
        if match_tok(token_iter.peek(), "+", src) || match_tok(token_iter.peek(), "-", src) {
            opped = true;
            add_op(token_iter, &mut sexp_node, src, simplified);
            term(token_iter, &mut sexp_node, src, simplified);
        } else {
            break;
        }
    }

    if !simplified{
        sexp_node.value = sexp_node.get_content(src);
    }
    if !opped {
        parent_node.add_child(sexp_node.reduce());
    }else {
        parent_node.add_child(sexp_node);
    }
}

fn term<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    let mut term_node = Node::new();
    let mut opped = false;

    if !simplified{
        term_node.n_type = NodeType::Term;
    }

    factor(token_iter, &mut term_node, src, simplified);

    loop {
        if match_tok(token_iter.peek(), "*", src) || match_tok(token_iter.peek(), "/", src) {
            opped = true;
            mulop(token_iter, &mut term_node, src, simplified);
            factor(token_iter, &mut term_node, src, simplified);
        } else {
            break;
        }
    }
    if !simplified{
        term_node.value = term_node.get_content(src);
    }
    if !opped {
        parent_node.add_child(term_node.reduce());
    } else {
        parent_node.add_child(term_node);
    }
}

fn factor<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
    simplified: bool,
) {
    let mut factor_node = Node::new();
    factor_node.n_type = NodeType::Factor;

    if let Some(tok) = token_iter.peek() {
        match tok {
            Token::NUMBER(_) => {
                if !simplified{
                    number(token_iter, &mut factor_node, src);
                } else {
                    number(token_iter, parent_node, src);
                }
            }
            Token::IDENTIFIER(_) => {
                if !simplified{
                    identifier(token_iter, &mut factor_node, src);
                } else {
                    identifier(token_iter, parent_node, src);
                }
            }
            Token::SYMBOL(_) => {
                if match_tok(token_iter.peek(), "(", src) {
                    if !simplified{
                        let mut open_brace_node = Node::new();
                        open_brace_node.n_type = NodeType::OpeningBrace;
                        open_brace_node.span = token_iter.next().unwrap().get_span();
                        open_brace_node.value = open_brace_node.get_content(src);
                        factor_node.add_child(open_brace_node);

                        exp(token_iter, &mut factor_node, src, simplified);
                        if match_tok(token_iter.peek(), ")", src) {
                            let mut close_brace_node = Node::new();
                            close_brace_node.n_type = NodeType::ClosingBrace;
                            close_brace_node.span = token_iter.next().unwrap().get_span();
                            close_brace_node.value = close_brace_node.get_content(src);
                            factor_node.add_child(close_brace_node);
                        } else {
                            //TODO: MissingBracket
                        }
                    } else {
                        exp(token_iter, parent_node, src, simplified);
                    }
                } else {
                    //TODO: IllegalFactor
                }
            }
            _ => {
                //TODO: IllegalFactor
            }
        }
    } else {
        //TODO: UnexpectedEOF
    }

    if !simplified {
        factor_node.value = factor_node.get_content(src);
        parent_node.add_child(factor_node);
    }
}

fn number<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
) {
    //Doesn't need simplification
    let mut number_node = Node::new();
    number_node.n_type = NodeType::Number;
    number_node.span = token_iter.next().unwrap().get_span();
    number_node.value = number_node.get_content(src);
    parent_node.add_child(number_node);
}

fn identifier<'a>(
    token_iter: &mut Peekable<Box<Iter<Token>>>,
    parent_node: &mut Node<'a>,
    src: &'a str,
) {
    //Doesn't need simplification
    let mut id_node = Node::new();
    id_node.n_type = NodeType::Identifier;
    id_node.span = token_iter.next().unwrap().get_span();
    id_node.value = id_node.get_content(src);
    parent_node.add_child(id_node);
}

fn match_tok(opt_tok: Option<&&Token>, res: &str, src: &str) -> bool {
    if let Some(tok) = opt_tok {
        if get_tok_content(tok, src) == res {
            return true;
        }
        false
    } else {
        false
    }
}

fn get_tok_content<'a>(tok: &Token, src: &'a str) -> &'a str {
    match *tok {
        Token::RESERVED((i0, i1))
        | Token::IDENTIFIER((i0, i1))
        | Token::NUMBER((i0, i1))
        | Token::COMMENT((i0, i1))
        | Token::SYMBOL((i0, i1)) => &src[i0..i1],
    }
}

fn add_error(parent_node: &mut Node, error_type: ErrorType, err_mess: String) {
    let mut err_node = Node::new();
    err_node.n_type = NodeType::Error(error_type, err_mess);
    parent_node.add_child(err_node);
}
