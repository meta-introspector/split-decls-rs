use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "emoji_poem" , vis = "pub" , hash = "232cff91")] pub fn emoji_poem (input : TokenStream) -> TokenStream { emoji_poetry :: emoji_poem_impl (input) }