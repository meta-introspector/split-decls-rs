use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Unescape a byte string literal"] # [doc = ""] # [doc = " Takes the contents of a byte string literal (without quotes)"] # [doc = " and produces a sequence of escaped bytes or errors,"] # [doc = " which are returned by invoking `callback`."] pub fn unescape_byte_str (src : & str , callback : impl FnMut (Range < usize > , Result < u8 , EscapeError >)) { < [u8] > :: unescape (src , callback) }