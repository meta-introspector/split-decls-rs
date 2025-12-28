use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "monster_check" , vis = "pub" , hash = "c02f748d")] pub fn monster_check (input : TokenStream) -> TokenStream { lmfdb_morph :: monster_check_impl (input) }