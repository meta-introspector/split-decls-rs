use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: replace_all_git_hub_actions");
# [proc_macro] # [decl (fn , name = "replace_all_git_hub_actions" , vis = "pub" , hash = "8dce1993")] pub fn replace_all_git_hub_actions (input : TokenStream) -> TokenStream { macros :: replace_all_git_hub_actions :: replace_all_git_hub_actions_impl (input) }
}