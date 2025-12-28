use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , U , const N : usize > PartialEq < & mut [U] > for SmallVec < T , N > where T : PartialEq < U > , { # [inline] fn eq (& self , other : & & mut [U]) -> bool { self [..] == other [..] } }
}