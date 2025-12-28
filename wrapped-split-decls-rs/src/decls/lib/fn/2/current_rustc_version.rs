use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: current_rustc_version");
# [proc_macro] pub fn current_rustc_version (input : TokenStream) -> TokenStream { current_version :: current_version (input) }
}