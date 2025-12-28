use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "find_rustc_source" , vis = "pub" , hash = "9ae2e0aa")] pub fn find_rustc_source (input : TokenStream) -> TokenStream { real_rustc_analysis :: find_rustc_source_impl (input) }