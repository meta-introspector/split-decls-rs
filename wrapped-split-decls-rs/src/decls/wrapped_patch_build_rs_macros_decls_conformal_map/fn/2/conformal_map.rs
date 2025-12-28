use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "conformal_map" , vis = "pub" , hash = "5a695325")] pub fn conformal_map (input : TokenStream) -> TokenStream { lmfdb_morph :: conformal_map_impl (input) }