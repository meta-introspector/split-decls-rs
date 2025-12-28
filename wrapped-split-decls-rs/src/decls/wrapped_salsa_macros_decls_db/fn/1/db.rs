use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro_attribute] pub fn db (args : TokenStream , input : TokenStream) -> TokenStream { db :: db (args , input) }