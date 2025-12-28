use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Hash for RealFileName { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . remapped_path_if_available () . hash (state) } }
}