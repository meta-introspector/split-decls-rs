use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl (fn , name = "foaf" , vis = "pub" , hash = "f6f07caf")] pub fn foaf (input : TokenStream) -> TokenStream { macros :: foaf :: foaf_impl (input) }
}