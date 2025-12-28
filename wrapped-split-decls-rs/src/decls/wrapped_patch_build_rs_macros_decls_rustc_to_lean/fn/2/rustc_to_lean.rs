use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "rustc_to_lean" , vis = "pub" , hash = "b7cffc51")] pub fn rustc_to_lean (input : TokenStream) -> TokenStream { lean4_proof :: rustc_to_lean_impl (input) }