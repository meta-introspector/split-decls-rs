use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "analyze_real_source" , vis = "pub" , hash = "45d79d1d")] pub fn analyze_real_source (input : TokenStream) -> TokenStream { real_rustc_analysis :: analyze_real_source_impl (input) }