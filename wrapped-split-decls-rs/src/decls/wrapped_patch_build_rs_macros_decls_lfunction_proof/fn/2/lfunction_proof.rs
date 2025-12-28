use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "lfunction_proof" , vis = "pub" , hash = "ce37759a")] pub fn lfunction_proof (input : TokenStream) -> TokenStream { lean4_proof :: lfunction_proof_impl (input) }