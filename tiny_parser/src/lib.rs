enum NodeType {
   STATEMENT(statement_type),
   OP(op_type),
   TERM,
   FACTOR,
   EXP,
   IDENTIFIER,
   ERROR,
}

enum statement_type {
    IF_STATEMENT,
    REPEAT_STATEMENT,
    WRITE_STATEMENT,
    READ_STATEMENT,
    ASSIGN_STATEMENT,
}

enum op_type {
    MULOP,
    ADDOP,
    COMPOP,
}

enum ErrorType {
    ILLEGAL_STATEMENT,
    MISSING_SEMICOLON,
    NON_ENDED_IF_STATEMENT,

}

fn parse() {
    program();
}

fn program() {
    stmt_seq();
}

fn stmt_seq() {
    stmt();

    loop {
        //TODO: match ; symbol
        stmt();
    }
}

fn stmt() {
    if_stmt();
    //OR
    repeat_stmt();
    //OR
    assign_stmt();
    //OR
    read_stmt();
    //OR
    write_stmt();
}

fn if_stmt() {
    //TODO: Match if token
    exp();
    //TODO: Match then token
    stmt_seq();
    
    //Optional Part starts here
    //TODO: Match else token
    stmt_seq();
    //TODO: Match end token
}

fn repeat_stmt() {
    //TODO: Match repeat token
    stmt_seq();
    //TODO: Match until token
    exp();
}

fn assign_stmt() {
    identifier();
    //TODO: Match ":=" symbol
    exp();
}

fn read_stmt() {
    //TODO: Match read token
    identifier();
}

fn write_stmt() {
    //TODO: Match write token
    exp();
}

fn exp() {
    simple_exp();
    //Optional part starts here
    comp_op();
    simple_exp();
}

fn comp_op() {
    //TODO: Match < or > or =
}

fn simple_exp() {
    term()

    loop {
        add_op();
        term();
    }
}

fn add_op() {
    //TODO: Match
}

fn term() {
    factor();

    loop {
        mulop();
        factor();
    }
}

fn mulop() {
    //TODO: Match * or /
}

fn factor() {
    //TODO: Match (
    exp();
    //TODO: Match )
    
    //OR

    number();

    //OR
    
    identifier();
}
