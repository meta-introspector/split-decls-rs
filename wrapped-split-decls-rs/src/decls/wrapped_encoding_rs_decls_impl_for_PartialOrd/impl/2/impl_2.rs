use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (test)] impl PartialOrd for Encoding { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { (self as * const Encoding as usize) . partial_cmp (& (other as * const Encoding as usize)) } }
}