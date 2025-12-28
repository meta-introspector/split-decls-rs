use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T , U , const N : usize > PartialEq < & [U] > for SmallVec < T , N > where T : PartialEq < U > , { # [inline] fn eq (& self , other : & & [U]) -> bool { self [..] == other [..] } }