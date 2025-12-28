use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > PartialOrd for ThinVec < T > where T : PartialOrd , { # [inline] fn partial_cmp (& self , other : & ThinVec < T >) -> Option < Ordering > { self [..] . partial_cmp (& other [..]) } }
}