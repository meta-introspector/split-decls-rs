use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: rustc_queries");
# [proc_macro] pub fn rustc_queries (input : TokenStream) -> TokenStream { query :: rustc_queries (input) }
}