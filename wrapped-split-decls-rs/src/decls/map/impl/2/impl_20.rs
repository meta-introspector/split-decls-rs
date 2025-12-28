use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , K , V > Index < & 'a K > for SsoHashMap < K , V > where K : Eq + Hash , { type Output = V ; # [inline] fn index (& self , key : & K) -> & V { self . get (key) . expect ("no entry found for key") } }
}