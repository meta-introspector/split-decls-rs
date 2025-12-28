use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn lines_with_ends (text : & str) -> LinesWithEnds { LinesWithEnds { text } }
}