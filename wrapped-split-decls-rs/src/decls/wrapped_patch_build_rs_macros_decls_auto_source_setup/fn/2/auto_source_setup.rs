use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "auto_source_setup" , vis = "pub" , hash = "c51a589c")] pub fn auto_source_setup (input : TokenStream) -> TokenStream { rustc_tracer :: auto_source_setup_impl (input) }
}