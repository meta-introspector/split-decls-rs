use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Position of original `expect!` in the source file."] # [derive (Debug)] pub struct Position { # [doc (hidden)] pub file : & 'static str , # [doc (hidden)] pub line : u32 , # [doc (hidden)] pub column : u32 , }