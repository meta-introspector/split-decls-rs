use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Punct > for TokenTree { fn from (g : Punct) -> Self { TokenTree :: Punct (g) } }
}