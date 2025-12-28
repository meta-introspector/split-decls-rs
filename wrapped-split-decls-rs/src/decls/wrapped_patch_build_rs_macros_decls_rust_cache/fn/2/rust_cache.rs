use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "rust_cache" , vis = "pub" , hash = "1e1d4711")] pub fn rust_cache (input : TokenStream) -> TokenStream { mkbuildrs :: rust_cache_impl (input) }