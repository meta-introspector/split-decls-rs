use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl (fn , name = "llm" , vis = "pub" , hash = "16e02522")] pub fn llm (input : TokenStream) -> TokenStream { macros :: llm :: llm_impl (input) }
}