use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Unescape a C string literal"] # [doc = ""] # [doc = " Takes the contents of a C string literal (without quotes)"] # [doc = " and produces a sequence of escaped MixedUnits or errors,"] # [doc = " which are returned by invoking `callback`."] pub fn unescape_c_str (src : & str , callback : impl FnMut (Range < usize > , Result < MixedUnit , EscapeError >) ,) { CStr :: unescape (src , callback) }
}