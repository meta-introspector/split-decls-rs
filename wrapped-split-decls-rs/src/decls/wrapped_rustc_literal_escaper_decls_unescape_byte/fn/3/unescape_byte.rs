use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: unescape_byte");
# [doc = " Unescape a byte literal"] # [doc = ""] # [doc = " Takes the contents of a byte literal (without quotes),"] # [doc = " and returns an unescaped byte or an error."] # [inline] pub fn unescape_byte (src : & str) -> Result < u8 , EscapeError > { < [u8] > :: unescape_single (& mut src . chars ()) }
}