use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl (fn , name = "codegen" , vis = "pub" , hash = "7a67ad6e")] pub fn codegen (input : TokenStream) -> TokenStream { macros :: codegen :: codegen_impl (input) }