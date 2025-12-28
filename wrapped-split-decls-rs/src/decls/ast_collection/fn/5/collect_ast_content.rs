use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Recursively collect the content of all nodes in the AST"] pub fn collect_ast_content (node : Node , source : & str) -> (String , usize) { let mut ast_output = String :: new () ; let mut stack = vec ! [node] ; let mut line_count = 0 ; while let Some (current_node) = stack . pop () { if current_node . is_named () && ! current_node . kind () . contains ("comment") { let node_text = & source [current_node . start_byte () .. current_node . end_byte ()] ; log :: debug ! ("Node type: {:?}, text: {:?}" , current_node . kind () , node_text) ; ast_output . push_str (& format ! ("{:?}\n" , current_node . kind ())) ; line_count += 1 ; } for child in current_node . children (& mut current_node . walk ()) { stack . push (child) ; } } (ast_output , line_count) }
}