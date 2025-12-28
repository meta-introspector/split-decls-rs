use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Unescape a string literal"] # [doc = ""] # [doc = " Takes the contents of a string literal (without quotes)"] # [doc = " and produces a sequence of escaped characters or errors,"] # [doc = " which are returned by invoking `callback`."] pub fn unescape_str (src : & str , callback : impl FnMut (Range < usize > , Result < char , EscapeError >)) { str :: unescape (src , callback) }