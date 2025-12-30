use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: llm_context");
# [proc_macro_attribute] pub fn llm_context (_args : TokenStream , input : TokenStream) -> TokenStream { input }
}