use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "dao_vote" , vis = "pub" , hash = "19f6b264")] pub fn dao_vote (input : TokenStream) -> TokenStream { dao_governance :: dao_vote_impl (input) }
}