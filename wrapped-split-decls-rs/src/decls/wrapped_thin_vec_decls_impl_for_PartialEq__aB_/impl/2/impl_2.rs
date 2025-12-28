use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , A , B > PartialEq < & 'a [B] > for ThinVec < A > where A : PartialEq < B > , { # [inline] fn eq (& self , other : & & 'a [B]) -> bool { self [..] == other [..] } }