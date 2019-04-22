use std::iter::Peekable;
use std::usize;
use tiny_lexer::lexer::{tokenize, Token, Span};
use std::slice::Iter;
use std::cmp::{max, min};

#[derive(Debug)]
enum NodeType {
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
    Error,
    Root,
    Null,
    Symbol,
}

#[derive(Debug)]
enum StmtType {
    IfStmt,
    RepeatStmt,
    WriteStmt,
    ReadStmt,
    AssignStmt,
}

#[derive(Debug)]
enum OpType {
    MulOp,
    AddOp,
    CompOp,
}


#[derive(Debug)]
enum ErrorType {
    IllegalStmt,
    MissingSemicolon,
    NonEndedIfStmt,
}

#[derive(Debug)]
pub struct Node {
    span: Span,
    n_type: NodeType,
    children: Vec<Node>,
}

impl Node {
    fn new() -> Node {
        Node {
            span: (usize::MAX,0),
            n_type: NodeType::Null,
            children: vec![]
        }
    }

    fn add_child(&mut self, child: Node){
        self.span.0 = min(self.span.0, child.span.0);
        self.span.1 = max(self.span.1, child.span.1);
        self.children.push(child);
    }
}

pub fn parse(src: &'static str) -> Node{
    let mut tokens: Vec<Token> = tokenize(src);
    let mut root = Node::new();
    root.n_type = NodeType::Root;
    let mut peekable_toks = Box::new(tokens.iter()).peekable();
    program(&mut peekable_toks, &mut root, src);
    root
}

fn program(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    let mut program_node = Node::new();
    program_node.n_type = NodeType::Program;
    stmt_seq(token_iter, &mut program_node, src);
    parent_node.add_child(program_node);
}

fn stmt_seq(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    let mut stmt_seq_node = Node::new();
    stmt_seq_node.n_type = NodeType::StmtSeq;

    stmt(token_iter, &mut stmt_seq_node, src);

    loop {
        //TODO: match ; symbol
        if match_tok(token_iter.peek(), ";", src){
            token_iter.next();
            stmt(token_iter, &mut stmt_seq_node, src);
        } else {
            break;
        }

    }

    parent_node.add_child(stmt_seq_node);
}

fn stmt(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    if let Some(&token) = token_iter.peek(){
        let mut stmt_node = Node::new();
        let stmt_type;
        match get_tok_content(token, src) {
            "if" => {
                stmt_type = StmtType::IfStmt;
                if_stmt(token_iter, &mut stmt_node, src);
            },
            "repeat" => {
                stmt_type = StmtType::RepeatStmt;
                repeat_stmt(token_iter, &mut stmt_node, src);
            },
            "read" => {
                stmt_type = StmtType::ReadStmt;
                read_stmt(token_iter, &mut stmt_node, src);
            },
            "write" => {
                stmt_type = StmtType::WriteStmt;
                write_stmt(token_iter, &mut stmt_node, src);
            }
            _ => {
                stmt_type = StmtType::AssignStmt;
                assign_stmt(token_iter, &mut stmt_node, src);
            }
        }
        stmt_node.n_type = NodeType::Stmt(stmt_type);
        parent_node.add_child(stmt_node);
    } else {
        //TODO: IllegalStmt Error
    }
}

fn if_stmt(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    //TODO: Match if token
    let mut if_node = Node::new();
    if_node.n_type = NodeType::Keyword;
    if_node.span = token_iter.next().unwrap().get_span();
    parent_node.add_child(if_node);

    exp(token_iter, parent_node, src);

    //TODO: Match then token
    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "then", src) {
            let mut then_node = Node::new();
            then_node.n_type = NodeType::Keyword;
            then_node.span = token_iter.next().unwrap().get_span();
            parent_node.add_child(then_node);
        } else {
            //TODO: MissingThenIfStmt
        }
    } else {
        //TODO: UnexpectedEOF
    }

    stmt_seq(token_iter, parent_node, src);

    //Optional Part starts here

    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "else", src) {
            let mut else_node = Node::new();
            else_node.n_type = NodeType::Keyword;
            else_node.span = token_iter.next().unwrap().get_span();
            parent_node.add_child(else_node);

            stmt_seq(token_iter, parent_node, src);
        }
    } else {
        //TODO: UnexpectedEOF
    }

    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "end", src) {
            let mut end_node = Node::new();
            end_node.n_type = NodeType::Keyword;
            end_node.span = token_iter.next().unwrap().get_span();
            parent_node.add_child(end_node);
        } else {
            //TODO: NonEndedIfStmt
        }
    } else {
        //TODO: UnexpectedEOF
    }
}

fn repeat_stmt(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    let mut repeat_node = Node::new();
    repeat_node.n_type = NodeType::Keyword;
    repeat_node.span = token_iter.next().unwrap().get_span();
    parent_node.add_child(repeat_node);

    stmt_seq(token_iter, parent_node, src);

    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "until", src) {
            let mut until_node = Node::new();
            until_node.n_type = NodeType::Keyword;
            until_node.span = token_iter.next().unwrap().get_span();
            parent_node.add_child(until_node);
        } else {
            //TODO: MissingUntilRepeatStmt
        }
    } else {
        //TODO: UnexpectedEOF
    }

    exp(token_iter, parent_node, src);
}

fn assign_stmt(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    identifier(token_iter, parent_node, src);

    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), ":=", src) {
            let mut ass_node = Node::new();
            ass_node.n_type = NodeType::Symbol;
            ass_node.span = token_iter.next().unwrap().get_span();
            parent_node.add_child(ass_node);
        } else {
            //TODO: MissingAssOperator
        }
    } else {
        //TODO: UnexpectedEOF
    }

    exp(token_iter, parent_node, src);
}

fn read_stmt(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    //TODO: Match read token
    let mut read_node = Node::new();
    read_node.n_type = NodeType::Keyword;
    read_node.span = token_iter.peek().unwrap().get_span();
    token_iter.next();
    parent_node.add_child(read_node);

    identifier(token_iter, parent_node, src);
}

fn write_stmt(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    //TODO: Match write token
    let mut write_node = Node::new();
    write_node.n_type = NodeType::Keyword;
    write_node.span = token_iter.next().unwrap().get_span();
    parent_node.add_child(write_node);

    exp(token_iter, parent_node, src);
}


//==============Start Of: Ops==============
fn add_op(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "+", src) || match_tok(token_iter.peek(), "-", src)  {
            let mut addop_node = Node::new();
            addop_node.n_type = NodeType::Op(OpType::AddOp);
            addop_node.span = token_iter.next().unwrap().get_span();
            parent_node.add_child(addop_node);
        } else {
            //TODO: MissingAddOperator
        }
    } else {
        //TODO: UnexpectedEOF
    }
}

fn mulop(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    //TODO: Match * or /
    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "*", src) || match_tok(token_iter.peek(), "/", src)  {
            let mut mulop_node = Node::new();
            mulop_node.n_type = NodeType::Op(OpType::MulOp);
            mulop_node.span = token_iter.next().unwrap().get_span();
            parent_node.add_child(mulop_node);
        } else {
            //TODO: MissingMulOperator
        }
    } else {
        //TODO: UnexpectedEOF
    }
}

fn comp_op(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    //TODO: Match < or =
    if token_iter.peek().is_some() {
        if match_tok(token_iter.peek(), "<", src) || match_tok(token_iter.peek(), "=", src)  {
            let mut compop_node = Node::new();
            compop_node.n_type = NodeType::Op(OpType::CompOp);
            compop_node.span = token_iter.next().unwrap().get_span();
            parent_node.add_child(compop_node);
        } else {
            //TODO: MissingCompOperator
        }
    } else {
        //TODO: UnexpectedEOF
    }
}
//==============End Of: Ops==============

fn exp(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    let mut exp_node = Node::new();
    exp_node.n_type = NodeType::Exp;

    simple_exp(token_iter, &mut exp_node, src);
    //Optional part starts here
    loop {
        if match_tok(token_iter.peek(), "<", src) || match_tok(token_iter.peek(), "=",src){
            comp_op(token_iter, &mut exp_node, src);
            simple_exp(token_iter, &mut exp_node, src);
        } else {
            break;
        }
    }
    parent_node.add_child(exp_node);
}


fn simple_exp(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    let mut sexp_node = Node::new();
    sexp_node.n_type = NodeType::SimplExp;

    term(token_iter, &mut sexp_node, src);

    loop {
        if match_tok(token_iter.peek(), "+", src) || match_tok(token_iter.peek(), "-",src) {
            add_op(token_iter, &mut sexp_node, src);
            term(token_iter, &mut sexp_node, src);
        } else {
            break;
        }
    }
    parent_node.add_child(sexp_node);
}



fn term(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    let mut term_node = Node::new();
    term_node.n_type = NodeType::Term;

    factor(token_iter, &mut term_node, src);

    loop {
        if match_tok(token_iter.peek(), "*", src) || match_tok(token_iter.peek(), "/",src) {
            mulop(token_iter, &mut term_node, src);
            factor(token_iter, &mut term_node, src);
        } else {
            break;
        }
    }
    parent_node.add_child(term_node);
}


fn factor(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    let mut factor_node = Node::new();
    factor_node.n_type = NodeType::Factor;

//    //TODO: Match (
//    exp();
//    //TODO: Match )
    //OR

    match token_iter.peek().unwrap() {
        Token::NUMBER(_) => {
            number(token_iter, &mut factor_node, src);
        },
        Token::IDENTIFIER(_) => {
            identifier(token_iter, &mut factor_node, src);
        },
        _ => {
            //TODO: IllegalFactor
        },
    }

    parent_node.add_child(factor_node);
    //OR

}

fn number(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    let mut number_node = Node::new();
    number_node.n_type = NodeType::Number;
    number_node.span = token_iter.peek().unwrap().get_span();
    token_iter.next();
    parent_node.add_child(number_node);
}

fn identifier(token_iter: &mut Peekable<Box<Iter<Token>>>, parent_node: &mut Node, src: &str) {
    let mut id_node = Node::new();
    id_node.n_type = NodeType::Identifier;
    id_node.span = token_iter.peek().unwrap().get_span();
    token_iter.next();
    parent_node.add_child(id_node);
}

fn match_tok (opt_tok: Option<&&Token>, res: &str, src: &str) -> bool {
    if let Some(tok) = opt_tok {
        if get_tok_content(tok, src) == res {
            return true;
        }
        false
    } else {
        false
    }
}

fn get_tok_content<'a>(tok: &Token, src: &'a str) -> &'a str{
    match *tok {
        Token::RESERVED((i0, i1)) |
        Token::IDENTIFIER((i0, i1)) |
        Token::NUMBER((i0, i1)) |
        Token::COMMENT((i0, i1)) |
        Token::SYMBOL((i0, i1)) => {
            &src[i0..i1]
        },
    }
}