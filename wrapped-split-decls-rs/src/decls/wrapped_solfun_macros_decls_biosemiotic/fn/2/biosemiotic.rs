use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl (fn , name = "biosemiotic" , vis = "pub" , hash = "81f81afc")] pub fn biosemiotic (input : TokenStream) -> TokenStream { macros :: biosemiotic :: biosemiotic_impl (input) }