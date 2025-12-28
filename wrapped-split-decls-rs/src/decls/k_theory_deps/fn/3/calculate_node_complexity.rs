use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn calculate_node_complexity (content : & str) -> usize { let complexity_indicators = ["impl" , "trait" , "struct" , "enum" , "fn" , "match" , "if" , "for" , "while" , "generic" , "where" , "async" , "unsafe" , "macro" , "derive"] ; complexity_indicators . par_iter () . map (| indicator | content . matches (indicator) . count ()) . sum () }
}