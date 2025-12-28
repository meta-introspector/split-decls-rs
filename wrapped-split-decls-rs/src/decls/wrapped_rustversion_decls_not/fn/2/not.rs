use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro_attribute] pub fn not (args : TokenStream , input : TokenStream) -> TokenStream { expand :: cfg ("not" , args , input) }