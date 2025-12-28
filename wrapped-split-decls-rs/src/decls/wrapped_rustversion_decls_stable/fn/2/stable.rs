use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro_attribute] pub fn stable (args : TokenStream , input : TokenStream) -> TokenStream { expand :: cfg ("stable" , args , input) }
}