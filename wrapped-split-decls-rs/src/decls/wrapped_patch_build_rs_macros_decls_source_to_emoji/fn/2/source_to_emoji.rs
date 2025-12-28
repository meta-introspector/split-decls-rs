use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "source_to_emoji" , vis = "pub" , hash = "303526a4")] pub fn source_to_emoji (input : TokenStream) -> TokenStream { rust_eigenmatrix :: source_to_emoji_impl (input) }
}