use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Hash for Utf8Path { fn hash < H : Hasher > (& self , state : & mut H) { for component in self . components () { component . hash (state) } } }
}