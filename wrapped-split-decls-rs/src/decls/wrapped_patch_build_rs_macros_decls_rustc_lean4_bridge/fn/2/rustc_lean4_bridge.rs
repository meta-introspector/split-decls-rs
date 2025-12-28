use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "rustc_lean4_bridge" , vis = "pub" , hash = "914b7981")] pub fn rustc_lean4_bridge (input : TokenStream) -> TokenStream { lean4_json :: rustc_lean4_bridge_impl (input) }