use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "metis_partition" , vis = "pub" , hash = "86020df6")] pub fn metis_partition (input : TokenStream) -> TokenStream { graph_partition :: metis_partition_impl (input) }