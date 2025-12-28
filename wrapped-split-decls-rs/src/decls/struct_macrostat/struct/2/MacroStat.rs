use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Default)] pub struct MacroStat { # [doc = " Number of uses of the macro."] pub uses : usize , # [doc = " Number of lines of code (when pretty-printed)."] pub lines : usize , # [doc = " Number of bytes of code (when pretty-printed)."] pub bytes : usize , }
}