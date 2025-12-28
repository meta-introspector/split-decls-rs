use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "emoji_to_math" , vis = "pub" , hash = "bdab912f")] pub fn emoji_to_math (input : TokenStream) -> TokenStream { emoji_poetry :: emoji_to_math_impl (input) }
}