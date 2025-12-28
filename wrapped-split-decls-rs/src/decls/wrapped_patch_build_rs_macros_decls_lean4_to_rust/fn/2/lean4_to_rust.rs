use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "lean4_to_rust" , vis = "pub" , hash = "266fd1cb")] pub fn lean4_to_rust (input : TokenStream) -> TokenStream { lean4_mirror :: lean4_to_rust_impl (input) }
}