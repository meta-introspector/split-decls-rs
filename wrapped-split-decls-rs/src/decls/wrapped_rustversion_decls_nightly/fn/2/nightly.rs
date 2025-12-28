use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro_attribute] pub fn nightly (args : TokenStream , input : TokenStream) -> TokenStream { expand :: cfg ("nightly" , args , input) }