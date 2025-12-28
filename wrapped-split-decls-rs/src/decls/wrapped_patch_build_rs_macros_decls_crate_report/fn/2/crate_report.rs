use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "crate_report" , vis = "pub" , hash = "7a2d32ce")] pub fn crate_report (input : TokenStream) -> TokenStream { rustc_ring :: crate_report_impl (input) }