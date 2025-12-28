use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , K , Q , V > Index < & 'a Q > for SortedMap < K , V > where K : Ord + Borrow < Q > , Q : Ord + ? Sized , { type Output = V ; fn index (& self , key : & Q) -> & Self :: Output { self . get (key) . expect ("no entry found for key") } }
}