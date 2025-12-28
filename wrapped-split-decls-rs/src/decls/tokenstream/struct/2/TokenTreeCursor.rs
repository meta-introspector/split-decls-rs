use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Debug)] pub struct TokenTreeCursor { stream : TokenStream , # [doc = " Points to the current token tree in the stream. In `TokenCursor::curr`,"] # [doc = " this can be any token tree. In `TokenCursor::stack`, this is always a"] # [doc = " `TokenTree::Delimited`."] index : usize , }
}