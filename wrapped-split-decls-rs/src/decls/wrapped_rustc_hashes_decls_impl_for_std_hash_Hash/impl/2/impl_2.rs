use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl std :: hash :: Hash for Hash128 { fn hash < H : std :: hash :: Hasher > (& self , h : & mut H) { h . write_u64 (self . truncate () . as_u64 ()) ; } }