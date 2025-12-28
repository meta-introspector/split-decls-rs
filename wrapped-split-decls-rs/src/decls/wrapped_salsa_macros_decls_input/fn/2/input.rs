use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: input");
# [proc_macro_attribute] pub fn input (args : TokenStream , input : TokenStream) -> TokenStream { input :: input (args , input) }
}