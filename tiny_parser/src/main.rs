mod lib;
use json::number::Number;
use json::JsonValue;
use lib::parse;

#[macro_use]
extern crate json;

fn main() {
    let src = "read{super cool} x; {cool}

            if 0 < x then

            fact := 1{really};

            repeat

            fact := fact * x;

            x := x - 1

            until x = 0;

            write fact

            end ";

    let simplified = true;

    let root = parse(src, simplified);

    let mut data = jsonify_node(root);
    println!("{}", data);
}

fn jsonify_node(node: lib::Node) -> JsonValue {
    let mut node_arr: Vec<JsonValue> = vec![];
    let mut nextnode_arr: Vec<JsonValue> = vec![];

    for child in node.children {
        node_arr.push(jsonify_node(child));
    }
    for child in node.nextstmt {
        nextnode_arr.push(jsonify_node(child));
    }

    object! {
        "type" => JsonValue::String(format!("{:?}", node.n_type)),
        "span" => array![node.span.0, node.span.1],
        "children" => JsonValue::Array(node_arr),
        "nextstmt" => JsonValue::Array(nextnode_arr),
    }
}
