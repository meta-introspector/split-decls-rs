use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Internable + ? Sized > Hash for Interned < T > { fn hash < H : Hasher > (& self , state : & mut H) { state . write_usize (Arc :: as_ptr (& self . arc) as * const () as usize) } }
}