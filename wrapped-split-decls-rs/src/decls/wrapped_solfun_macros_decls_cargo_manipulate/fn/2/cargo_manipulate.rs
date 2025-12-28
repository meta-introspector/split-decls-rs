use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl (fn , name = "cargo_manipulate" , vis = "pub" , hash = "807cd9e0")] pub fn cargo_manipulate (input : TokenStream) -> TokenStream { macros :: cargo_manipulate :: cargo_manipulate_impl (input) }