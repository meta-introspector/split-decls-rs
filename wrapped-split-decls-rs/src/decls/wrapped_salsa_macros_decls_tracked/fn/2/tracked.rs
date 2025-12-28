use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro_attribute] pub fn tracked (args : TokenStream , input : TokenStream) -> TokenStream { tracked :: tracked (args , input) }
}