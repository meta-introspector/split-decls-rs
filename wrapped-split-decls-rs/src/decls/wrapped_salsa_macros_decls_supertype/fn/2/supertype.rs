use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: supertype");
# [proc_macro_derive (Supertype)] pub fn supertype (input : TokenStream) -> TokenStream { supertype :: supertype (input) }
}