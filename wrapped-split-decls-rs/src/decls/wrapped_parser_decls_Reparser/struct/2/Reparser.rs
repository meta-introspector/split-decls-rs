use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A parsing function for a specific braced-block."] pub struct Reparser (fn (& mut parser :: Parser < '_ >)) ;
}