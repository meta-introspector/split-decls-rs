use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " The same as `eprint!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn ceprint (input : TokenStream) -> TokenStream { get_macro ("eprint" , input , false) }
}