use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "stark_proof" , vis = "pub" , hash = "d5fae4a8")] pub fn stark_proof (input : TokenStream) -> TokenStream { zk_proof :: stark_proof_impl (input) }