use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl (fn , name = "github" , vis = "pub" , hash = "b2981013")] pub fn github (input : TokenStream) -> TokenStream { macros :: github :: github_impl (input) }