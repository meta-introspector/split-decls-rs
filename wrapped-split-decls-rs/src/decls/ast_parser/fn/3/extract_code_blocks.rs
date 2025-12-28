use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: extract_code_blocks");
pub fn extract_code_blocks (tree : Tree , source : & str , threshold : usize) -> Vec < CodeBlockRef > { let mut cursor = tree . walk () ; let mut code_blocks = Vec :: new () ; traverse_tree (& mut cursor , source , & mut code_blocks , threshold , 0 , TREE_PARSING_MAX_DEPTH , None) ; code_blocks }
}