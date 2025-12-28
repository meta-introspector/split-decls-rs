use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < A , B > PartialEq < Vec < B > > for ThinVec < A > where A : PartialEq < B > , { # [inline] fn eq (& self , other : & Vec < B >) -> bool { self [..] == other [..] } }
}