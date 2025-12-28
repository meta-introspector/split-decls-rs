use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl (fn , name = "fork_all_repos" , vis = "pub" , hash = "4ac3f475")] pub fn fork_all_repos (input : TokenStream) -> TokenStream { macros :: fork_all_repos :: fork_all_repos_impl (input) }
}