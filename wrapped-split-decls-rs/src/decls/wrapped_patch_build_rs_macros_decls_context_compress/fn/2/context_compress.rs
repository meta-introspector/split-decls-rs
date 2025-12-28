use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "compress" , vis = "pub" , hash = "3be315c8")] pub fn context_compress (input : TokenStream) -> TokenStream { context_knapsack :: context_compress_impl (input) }