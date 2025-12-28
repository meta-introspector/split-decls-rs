use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl (fn , name = "branch" , vis = "pub" , hash = "c2bf67ba")] pub fn branch (input : TokenStream) -> TokenStream { macros :: branch :: branch_impl (input) }