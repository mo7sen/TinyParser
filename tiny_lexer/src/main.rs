mod lexer;
use lexer::tokenize;
use lexer::Token;
use lexer::Token::{COMMENT, IDENTIFIER, NUMBER, RESERVED, SYMBOL};

fn main() {
    let file_contents = "{ Sample program in TINY language computes factorial }

    read x; {input an integer }

    if 0 < x then { don't compute if x <= 0 }

    fact := 1;

    repeat

    fact := fact * x;

    x := x - 1

    until x = 0;

    write fact { output factorial of x }

    end ";
    //"{ Sample program in TINY language – computes factorial }
    //read x; {input an integer }
    //if 0 < x then { don’t compute if x <= 0 }
    //fact := 1;
    //repeat
    //fact := fact * x;
    //x := x - 1
    //until x = 0;
    //write fact { output factorial of x }
    //end";

    let tokens = tokenize(file_contents, true);
    //    println!("{:#?}", tokens);
    for tok in tokens {
        match tok {
            RESERVED((i0, i1)) | IDENTIFIER((i0, i1)) | IDENTIFIER((i0, i1)) | NUMBER((i0, i1))
            | COMMENT((i0, i1)) | SYMBOL((i0, i1)) => {
                println!("{}", &file_contents[i0..i1]);
            }
        }
    }
}
