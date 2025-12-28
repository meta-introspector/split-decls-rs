use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "sat_solve_unity" , vis = "pub" , hash = "25395982")] pub fn sat_solve_unity (input : TokenStream) -> TokenStream { sat_lfunction :: sat_solve_unity_impl (input) }
}