use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (target_pointer_width = "64")] impl Hash for DefId { fn hash < H : Hasher > (& self , h : & mut H) { (((self . krate . as_u32 () as u64) << 32) | (self . index . as_u32 () as u64)) . hash (h) } }
}