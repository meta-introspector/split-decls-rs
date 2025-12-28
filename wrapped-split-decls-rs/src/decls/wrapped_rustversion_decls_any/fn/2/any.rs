use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro_attribute] pub fn any (args : TokenStream , input : TokenStream) -> TokenStream { expand :: cfg ("any" , args , input) }
}