use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "lattice_path" , vis = "pub" , hash = "af81a817")] pub fn lattice_path (input : TokenStream) -> TokenStream { macro_lattice :: lattice_path_impl (input) }
}