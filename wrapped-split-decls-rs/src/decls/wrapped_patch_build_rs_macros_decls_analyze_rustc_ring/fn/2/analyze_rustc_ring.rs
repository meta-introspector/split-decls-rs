use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "analyze_rustc_ring" , vis = "pub" , hash = "a38b31bb")] pub fn analyze_rustc_ring (input : TokenStream) -> TokenStream { rustc_ring :: analyze_rustc_ring_impl (input) }