use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , const N : usize > From < Vec < T > > for SmallVec < T , N > { fn from (array : Vec < T >) -> Self { Self :: from_vec (array) } }
}