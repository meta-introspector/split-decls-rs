use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "token" , vis = "pub" , hash = "280ff363")] pub fn token_weight (input : TokenStream) -> TokenStream { context_knapsack :: token_weight_impl (input) }
}