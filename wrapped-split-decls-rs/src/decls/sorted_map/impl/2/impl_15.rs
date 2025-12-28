use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , K , Q , V > IndexMut < & 'a Q > for SortedMap < K , V > where K : Ord + Borrow < Q > , Q : Ord + ? Sized , { fn index_mut (& mut self , key : & Q) -> & mut Self :: Output { self . get_mut (key) . expect ("no entry found for key") } }
}