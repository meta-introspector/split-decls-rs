use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < TokenTree > for TokenStream { fn from (token : TokenTree) -> Self { TokenStream :: _new (imp :: TokenStream :: from (token)) } }
}