use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : Copy > TokenTree < S > { pub fn first_span (& self) -> S { match self { TokenTree :: Leaf (l) => * l . span () , TokenTree :: Subtree (s) => s . delimiter . open , } } }
}