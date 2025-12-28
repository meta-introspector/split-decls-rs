use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "sat_group" , vis = "pub" , hash = "d93ebe48")] pub fn sat_group (input : TokenStream) -> TokenStream { graph_partition :: sat_group_impl (input) }
}