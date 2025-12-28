use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (test)] impl Ord for Encoding { fn cmp (& self , other : & Self) -> Ordering { (self as * const Encoding as usize) . cmp (& (other as * const Encoding as usize)) } }