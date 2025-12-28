use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "hott_morph" , vis = "pub" , hash = "65c4cf26")] pub fn hott_morph (input : TokenStream) -> TokenStream { lmfdb_morph :: hott_morph_impl (input) }
}