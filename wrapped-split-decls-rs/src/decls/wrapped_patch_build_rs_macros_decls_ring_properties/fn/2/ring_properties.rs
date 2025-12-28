use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "ring_properties" , vis = "pub" , hash = "2c553716")] pub fn ring_properties (input : TokenStream) -> TokenStream { rustc_ring :: ring_properties_impl (input) }