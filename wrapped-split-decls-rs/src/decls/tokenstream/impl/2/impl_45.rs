use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 't > TokenStreamIter < 't > { fn new (stream : & 't TokenStream) -> Self { TokenStreamIter { stream , index : 0 } } pub fn peek (& self) -> Option < & 't TokenTree > { self . stream . 0 . get (self . index) } }
}