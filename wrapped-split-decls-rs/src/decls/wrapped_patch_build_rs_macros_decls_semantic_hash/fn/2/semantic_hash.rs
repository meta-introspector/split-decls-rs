use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "semantic_hash" , vis = "pub" , hash = "a92938c2")] pub fn semantic_hash (input : TokenStream) -> TokenStream { duplicate_analysis :: semantic_hash_impl (input) }