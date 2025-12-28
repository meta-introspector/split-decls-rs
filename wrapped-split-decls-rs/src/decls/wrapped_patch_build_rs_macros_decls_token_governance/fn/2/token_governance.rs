use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "token_governance" , vis = "pub" , hash = "52305b31")] pub fn token_governance (input : TokenStream) -> TokenStream { dao_governance :: token_governance_impl (input) }