use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct SourceFileAndLine { pub sf : Arc < SourceFile > , # [doc = " Index of line, starting from 0."] pub line : usize , }
}