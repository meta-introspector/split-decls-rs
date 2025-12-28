use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] pub fn current_rustc_version (input : TokenStream) -> TokenStream { current_version :: current_version (input) }