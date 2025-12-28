use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "compiler_macro" , vis = "pub" , hash = "e7c41b14")] pub fn compiler_macro (input : TokenStream) -> TokenStream { quine_relay :: compiler_macro_impl (input) }