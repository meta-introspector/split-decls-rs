use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: dep_path");
# [proc_macro] pub fn dep_path (input : TokenStream) -> TokenStream { macros :: dep_path_impl (input) }
}