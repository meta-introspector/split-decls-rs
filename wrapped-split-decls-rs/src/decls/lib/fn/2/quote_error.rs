use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: quote_error");
# [proc_macro] pub fn quote_error (input : TokenStream) -> TokenStream { input }
}