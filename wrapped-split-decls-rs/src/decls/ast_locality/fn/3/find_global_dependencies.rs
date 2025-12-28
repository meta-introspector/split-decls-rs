use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn find_global_dependencies (content : & str) -> Vec < String > { let global_patterns = ["std::" , "extern " , "use std" , "use anyhow" , "use serde" , "use rayon" , "HashMap" , "Vec" , "Result" , "Option" , "Box" , "Arc" , "Mutex"] ; global_patterns . par_iter () . filter (| pattern | content . contains (* pattern)) . map (| s | s . to_string ()) . collect () }
}