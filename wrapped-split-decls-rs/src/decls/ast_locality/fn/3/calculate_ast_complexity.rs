use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: calculate_ast_complexity");
fn calculate_ast_complexity (content : & str) -> usize { let complexity_indicators = ["{" , "}" , "(" , ")" , "[" , "]" , "match" , "if" , "for" , "impl"] ; complexity_indicators . par_iter () . map (| indicator | content . matches (indicator) . count ()) . sum () }
}