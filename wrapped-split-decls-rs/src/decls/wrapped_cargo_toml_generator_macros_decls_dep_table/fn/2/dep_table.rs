use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: dep_table");
# [proc_macro] pub fn dep_table (input : TokenStream) -> TokenStream { macros :: dep_table_impl (input) }
}