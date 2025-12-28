use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Parsed token."] # [doc = " It doesn't contain information about data that has been parsed,"] # [doc = " only the type of the token and its size."] # [derive (Debug)] pub struct Token { pub kind : TokenKind , pub len : u32 , }
}