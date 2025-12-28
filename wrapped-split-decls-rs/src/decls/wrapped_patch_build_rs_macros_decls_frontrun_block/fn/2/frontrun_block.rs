use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "frontrun_block" , vis = "pub" , hash = "35db1ddf")] pub fn frontrun_block (input : TokenStream) -> TokenStream { mev_protection :: frontrun_block_impl (input) }