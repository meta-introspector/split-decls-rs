use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct Splice < 'a , I : Iterator + 'a , const N : usize > { drain : Drain < 'a , I :: Item , N > , replace_with : I , }
}