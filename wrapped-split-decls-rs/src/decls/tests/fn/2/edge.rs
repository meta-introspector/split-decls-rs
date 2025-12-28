use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: edge");
fn edge (from : usize , to : usize , label : & 'static str , style : Style) -> Edge { Edge { from , to , label , style } }
}