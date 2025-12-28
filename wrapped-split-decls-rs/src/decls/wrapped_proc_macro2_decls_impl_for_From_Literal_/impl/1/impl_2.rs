use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Literal > for TokenTree { fn from (g : Literal) -> Self { TokenTree :: Literal (g) } }
}