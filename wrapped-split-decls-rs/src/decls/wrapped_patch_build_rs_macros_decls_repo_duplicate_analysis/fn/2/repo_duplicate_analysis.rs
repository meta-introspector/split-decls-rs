use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "ca" , vis = "pub" , hash = "5eaef638")] pub fn repo_duplicate_analysis (input : TokenStream) -> TokenStream { repo_analysis :: repo_duplicate_analysis_impl (input) }
}