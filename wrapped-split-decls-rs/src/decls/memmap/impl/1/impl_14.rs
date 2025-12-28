use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl DerefMut for MmapMut { # [inline] fn deref_mut (& mut self) -> & mut [u8] { & mut self . 0 } }
}