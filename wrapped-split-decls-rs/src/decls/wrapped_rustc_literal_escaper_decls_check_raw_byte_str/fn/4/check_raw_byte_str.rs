use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: check_raw_byte_str");
# [doc = " Check a raw byte string literal for validity"] # [doc = ""] # [doc = " Takes the contents of a raw byte string literal (without quotes)"] # [doc = " and produces a sequence of bytes or errors,"] # [doc = " which are returned by invoking `callback`."] # [doc = " NOTE: Does no escaping, but produces errors for bare carriage return ('\\r')."] pub fn check_raw_byte_str (src : & str , callback : impl FnMut (Range < usize > , Result < u8 , EscapeError >)) { < [u8] > :: check_raw (src , callback) ; }
}