use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Hash for Ident { fn hash < H : Hasher > (& self , state : & mut H) { self . name . hash (state) ; self . span . ctxt () . hash (state) ; } }