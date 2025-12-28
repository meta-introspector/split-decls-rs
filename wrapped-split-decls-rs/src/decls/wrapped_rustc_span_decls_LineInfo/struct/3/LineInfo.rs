use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct LineInfo { # [doc = " Index of line, starting from 0."] pub line_index : usize , # [doc = " Column in line where span begins, starting from 0."] pub start_col : CharPos , # [doc = " Column in line where span ends, starting from 0, exclusive."] pub end_col : CharPos , }
}