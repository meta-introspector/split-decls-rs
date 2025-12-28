use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl (fn , name = "bug" , vis = "pub" , hash = "363cb3e8")] pub fn bug (input : TokenStream) -> TokenStream { macros :: bug :: bug_impl (input) }