use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "real_rustc_analysis" , vis = "pub" , hash = "27c9551f")] pub fn real_rustc_analysis (input : TokenStream) -> TokenStream { real_data_analysis :: real_rustc_analysis_impl (input) }
}