use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Ident > for TokenTree { fn from (g : Ident) -> Self { TokenTree :: Ident (g) } }
}