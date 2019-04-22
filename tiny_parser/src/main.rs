mod lib;
use lib::parse;

fn main() {
    parse(
        "read{super cool} x; {cool}

            if ((((0 < x)))) then

            fact := 1{really};

            repeat

            fact := fact * x;

            x := x - 1

            until x = 0;

            write fact

            end ",
    )
    .print_dbg(String::new());
}
