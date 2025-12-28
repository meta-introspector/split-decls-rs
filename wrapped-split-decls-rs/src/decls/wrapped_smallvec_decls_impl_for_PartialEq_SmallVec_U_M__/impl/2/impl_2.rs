use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T , U , const N : usize , const M : usize > PartialEq < SmallVec < U , M > > for SmallVec < T , N > where T : PartialEq < U > , { # [inline] fn eq (& self , other : & SmallVec < U , M >) -> bool { self . as_slice () . eq (other . as_slice ()) } }