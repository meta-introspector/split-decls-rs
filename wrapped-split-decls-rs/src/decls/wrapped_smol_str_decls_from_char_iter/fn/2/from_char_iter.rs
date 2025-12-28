use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: from_char_iter");
# [inline] fn from_char_iter (iter : impl Iterator < Item = char >) -> SmolStr { from_buf_and_chars ([0 ; _] , 0 , iter) }
}