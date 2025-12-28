use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl (fn , name = "twitter" , vis = "pub" , hash = "758954c5")] pub fn twitter (input : TokenStream) -> TokenStream { macros :: twitter :: twitter_impl (input) }
}