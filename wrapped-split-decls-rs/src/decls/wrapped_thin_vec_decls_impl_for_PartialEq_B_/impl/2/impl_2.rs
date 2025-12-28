use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < A , B > PartialEq < [B] > for ThinVec < A > where A : PartialEq < B > , { # [inline] fn eq (& self , other : & [B]) -> bool { self [..] == other [..] } }
}