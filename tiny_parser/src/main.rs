mod lib;
use json::number::Number;
use json::JsonValue;
use lib::parse;

#[macro_use]
extern crate json;

fn main() {
    let src = "read{super cool} x; {cool}

            if x then

            fact := 1{really};

            repeat

            fact := fact * x;

            x := x - 1

            until x = 0;

            write fact;

            end ";
    let simplified = true;
    let root = parse(src, simplified);

    let mut data = jsonify_node(root);

    println!("{}", data);
    //    root.print_dbg(String::new());
}

fn jsonify_node(node: lib::Node) -> JsonValue {
    let mut node_arr: Vec<JsonValue> = vec![];

    for child in node.children {
        node_arr.push(jsonify_node(child));
    }

    object! {
        "type" => JsonValue::String(format!("{:?}", node.n_type)),
        "span" => array![node.span.0, node.span.1],
        "value" => JsonValue::String(node.value.to_string()),
        "children" => JsonValue::Array(node_arr),
    }
}
