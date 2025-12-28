use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Hash for Encoding { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { (self as * const Encoding) . hash (state) ; } }
}