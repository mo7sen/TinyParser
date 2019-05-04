mod lib;
use json::JsonValue;
use lib::parse;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[macro_use]
extern crate json;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut in_path = Path::new(&args[1]);
    let mut out_path = Path::new(&args[2]);
    let simplified = match FromStr::from_str(&args[3]) {
        Ok(n) => n,
        _ => true,
    };
    let mut nigga = false;
    if args.len() > 4 {
        if &args[4] == "nigger" {
            nigga = true;
        }
    }

    let src = fs::read_to_string(in_path).expect("Incorrect + Path");
    let mut root = parse(&src, simplified);
    root.get_content(&src);
    if simplified {
        root = root.children.remove(0);
    }

    let data = jsonify_node(root, simplified, nigga);
    fs::write(out_path, format!("{}", data));
}

use lib::NodeType;

fn jsonify_node(node: lib::Node, simplified: bool, nigga: bool) -> JsonValue {
    let mut node_arr: Vec<JsonValue> = vec![];
    let class = if nigga {
        "nigger"
    } else if let NodeType::Stmt(_) = node.n_type {
        "stmt"
    } else if let NodeType::Error(_, _) = node.n_type {
        "error"
    } else {
        "normie"
    }
    .to_string();

    for child in node.children {
        node_arr.push(jsonify_node(child, simplified, nigga));
    }

    let typ = node.n_type.clone();

    let text = if simplified {
        object! {
            "value" => JsonValue::String(node.value),
        }
    } else {
        object! {
            "type" => JsonValue::String(
            match node.n_type {
                NodeType::Error(e,_) => format!("{:?}", e),
                _ => format!("{:?}", node.n_type)
            })
        }
    };

    object! {
        "text" => text,
        "pseudo" => if let NodeType::Null = typ {
            JsonValue::Boolean(true)
        } else {
            JsonValue::Boolean(false)
        },
        "span" => array![node.span.0, node.span.1],
        "children" => JsonValue::Array(node_arr),
        "HTMLclass"=> JsonValue::String(class),
    }
}

//fn jsonify_all(node: lib::Node, vec: &mut Vec<JsonValue>) {
//    let class = if let NodeType::Stmt(_) = node.n_type {
//        "stmt"
//    } else {
//        "normie"
//    }.to_string();
//
//    for child in node.children {
//        jsonify_all(child, vec);
//    }
//    for child in node.nextstmt {
//        jsonify_all(child, vec);
//    }
//
//    vec.push(object! {
//        //"text" => text,
//        "type" => JsonValue::String(format!("{:?}", node.n_type)),
//        "span" => array![node.span.0, node.span.1],
//        "pos" => array![node.x, node.y],
//        "htmlclass"=> JsonValue::String(class),
//    });
//}

//pub fn legacy_spacing(node: &mut lib::Node, sibling_hspacing: i32, sibling_vspacing: i32) {
//    let child_count = node.children.len() as i32;
//    let (curr_x, curr_y) = (node.x, node.y);
//    let mut starting_factor = child_count / -2;
//    for child in node.children.iter_mut() {
//        if starting_factor == 0 && child_count % 2 == 0 {
//            starting_factor += 1;
//        }
//        child.x = (curr_x + (starting_factor * sibling_hspacing));
//        if let NodeType::Stmt(_) = child.n_type {
//            child.y = (curr_y + (sibling_vspacing as f64 * 1.5) as i32);
//        } else {
//            child.y = (curr_y + sibling_vspacing);
//        }
//        starting_factor += 1;
//        legacy_spacing(child, (sibling_hspacing as f64 / (3 as f64 / 2 as f64)) as i32, sibling_vspacing);
//    }
//
//    for sib in node.nextstmt.iter_mut() {
//        sib.y = (curr_y);
//        sib.x = (curr_x + (6  * sibling_hspacing ));
//        legacy_spacing(sib,
//                        sibling_hspacing
//                       , sibling_vspacing);
//    }
//}
