use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl (fn , name = "osm" , vis = "pub" , hash = "ca96da81")] pub fn osm (input : TokenStream) -> TokenStream { macros :: osm :: osm_impl (input) }
}