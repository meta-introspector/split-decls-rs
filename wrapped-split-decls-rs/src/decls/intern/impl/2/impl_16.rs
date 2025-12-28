use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T > Hash for Interned < 'a , T > where T : Hash , { # [inline] fn hash < H : Hasher > (& self , s : & mut H) { ptr :: hash (self . 0 , s) } }