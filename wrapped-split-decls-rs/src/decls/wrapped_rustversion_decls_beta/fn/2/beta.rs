use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro_attribute] pub fn beta (args : TokenStream , input : TokenStream) -> TokenStream { expand :: cfg ("beta" , args , input) }