use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > DoubleEndedIterator for Iter < 'a > { # [inline] fn next_back (& mut self) -> Option < & 'a str > { self . inner . next_back () . map (| component | component . as_str ()) } }
}