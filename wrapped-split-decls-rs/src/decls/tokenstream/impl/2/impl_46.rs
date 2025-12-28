use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 't > Iterator for TokenStreamIter < 't > { type Item = & 't TokenTree ; fn next (& mut self) -> Option < & 't TokenTree > { self . stream . 0 . get (self . index) . map (| tree | { self . index += 1 ; tree }) } }
}