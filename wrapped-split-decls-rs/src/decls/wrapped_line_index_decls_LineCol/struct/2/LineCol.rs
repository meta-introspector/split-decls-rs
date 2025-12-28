use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " `(line, column)` information in the native, UTF-8 encoding."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct LineCol { # [doc = " Zero-based."] pub line : u32 , # [doc = " Zero-based UTF-8 offset."] pub col : u32 , }
}