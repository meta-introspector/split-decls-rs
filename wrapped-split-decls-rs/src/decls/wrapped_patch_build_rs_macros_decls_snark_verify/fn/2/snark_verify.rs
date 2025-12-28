use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "snark_verify" , vis = "pub" , hash = "2863bbef")] pub fn snark_verify (input : TokenStream) -> TokenStream { zk_proof :: snark_verify_impl (input) }