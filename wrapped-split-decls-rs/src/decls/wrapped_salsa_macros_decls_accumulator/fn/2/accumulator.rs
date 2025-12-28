use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro_attribute] pub fn accumulator (args : TokenStream , input : TokenStream) -> TokenStream { accumulator :: accumulator (args , input) }
}