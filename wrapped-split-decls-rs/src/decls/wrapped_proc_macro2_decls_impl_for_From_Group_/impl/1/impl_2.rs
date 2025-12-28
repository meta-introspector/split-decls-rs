use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Group > for TokenTree { fn from (g : Group) -> Self { TokenTree :: Group (g) } }
}