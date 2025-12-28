use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro_attribute] pub fn interned (args : TokenStream , input : TokenStream) -> TokenStream { interned :: interned (args , input) }
}