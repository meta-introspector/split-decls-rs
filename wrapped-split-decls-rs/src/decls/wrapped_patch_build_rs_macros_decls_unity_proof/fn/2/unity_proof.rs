use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "unity_proof" , vis = "pub" , hash = "7c9d4384")] pub fn unity_proof (input : TokenStream) -> TokenStream { sat_lfunction :: unity_proof_impl (input) }
}