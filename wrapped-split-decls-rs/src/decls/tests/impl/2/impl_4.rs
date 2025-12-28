use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialOrd for S { fn partial_cmp (& self , other : & S) -> Option < Ordering > { assert_ne ! (self . 0 , other . 0) ; self . 0 . partial_cmp (& other . 0) } }
}