use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "patch_rust" , vis = "pub" , hash = "d60f6cd9")] pub fn patch_rust (input : TokenStream) -> TokenStream { rust_nix :: patch_rust_impl (input) }
}