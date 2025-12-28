use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " The same as `println!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn cprintln (input : TokenStream) -> TokenStream { get_macro ("println" , input , false) }