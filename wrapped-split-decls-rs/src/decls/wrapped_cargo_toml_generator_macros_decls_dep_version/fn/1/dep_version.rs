use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] pub fn dep_version (input : TokenStream) -> TokenStream { macros :: dep_version_impl (input) }