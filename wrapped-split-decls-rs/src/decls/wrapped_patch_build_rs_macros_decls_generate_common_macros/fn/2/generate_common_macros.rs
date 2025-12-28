use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "generate_common_macros" , vis = "pub" , hash = "7a7ed877")] pub fn generate_common_macros (input : TokenStream) -> TokenStream { macro_generator :: generate_common_macros_impl (input) }