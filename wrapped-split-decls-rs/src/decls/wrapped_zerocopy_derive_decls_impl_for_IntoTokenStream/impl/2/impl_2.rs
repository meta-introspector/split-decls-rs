use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl IntoTokenStream for Result < TokenStream , Error > { fn into_ts (self) -> TokenStream { match self { Ok (ts) => ts , Err (err) => err . to_compile_error () , } } }
}