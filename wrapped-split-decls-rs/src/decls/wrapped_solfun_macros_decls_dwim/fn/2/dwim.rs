use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl (fn , name = "dwim" , vis = "pub" , hash = "20bca028")] pub fn dwim (input : TokenStream) -> TokenStream { macros :: dwim :: dwim_impl (input) }
}