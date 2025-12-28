use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < P , T : Tag > Hash for TaggedRef < '_ , P , T > { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . packed . hash (state) ; } }
}