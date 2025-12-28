use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Iterator > DoubleEndedIterator for Splice < '_ , I > { fn next_back (& mut self) -> Option < Self :: Item > { self . drain . next_back () } }
}