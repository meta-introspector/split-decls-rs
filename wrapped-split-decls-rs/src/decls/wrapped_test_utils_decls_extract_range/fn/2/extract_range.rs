use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Infallible version of `try_extract_range()`."] pub fn extract_range (text : & str) -> (TextRange , String) { match try_extract_range (text) { None => panic ! ("text should contain cursor marker") , Some (result) => result , } }