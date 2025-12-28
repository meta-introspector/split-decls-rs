use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " The same as `write!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn cwrite (input : TokenStream) -> TokenStream { get_macro ("write" , input , true) }
}