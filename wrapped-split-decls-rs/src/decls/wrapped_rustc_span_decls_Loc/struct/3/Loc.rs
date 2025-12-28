use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A source code location used for error reporting."] # [derive (Debug , Clone)] pub struct Loc { # [doc = " Information about the original source."] pub file : Arc < SourceFile > , # [doc = " The (1-based) line number."] pub line : usize , # [doc = " The (0-based) column offset."] pub col : CharPos , # [doc = " The (0-based) column offset when displayed."] pub col_display : usize , }
}