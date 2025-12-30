use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: llm_error_message");
# [proc_macro_attribute] pub fn llm_error_message (_args : TokenStream , input : TokenStream) -> TokenStream { input }
}