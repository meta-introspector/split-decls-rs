use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl IntoTokenStream for Result < TokenStream , Error > { fn into_ts (self) -> TokenStream { match self { Ok (ts) => ts , Err (err) => err . to_compile_error () , } } }