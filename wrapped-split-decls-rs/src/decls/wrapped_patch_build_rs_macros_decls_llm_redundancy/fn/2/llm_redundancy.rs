use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "llm_redundancy" , vis = "pub" , hash = "f4c786ab")] pub fn llm_redundancy (input : TokenStream) -> TokenStream { duplicate_analysis :: llm_redundancy_impl (input) }
}