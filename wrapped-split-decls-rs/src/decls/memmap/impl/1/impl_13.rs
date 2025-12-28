use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Deref for MmapMut { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { & self . 0 } }
}