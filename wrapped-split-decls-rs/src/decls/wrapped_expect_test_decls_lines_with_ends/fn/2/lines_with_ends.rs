use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: lines_with_ends");
fn lines_with_ends (text : & str) -> LinesWithEnds { LinesWithEnds { text } }
}