use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "decl" , vis = "pub" , hash = "9cec6423")] pub fn extract_decl (input : TokenStream) -> TokenStream { rust_nix :: extract_decl_impl (input) }
}