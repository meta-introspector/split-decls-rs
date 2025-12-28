use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Hash for Uuid { fn hash < H : Hasher > (& self , state : & mut H) { state . write (& self . 0) ; } }
}