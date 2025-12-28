use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ToTokens for BindingInfo < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { self . binding . to_tokens (tokens) ; } }
}