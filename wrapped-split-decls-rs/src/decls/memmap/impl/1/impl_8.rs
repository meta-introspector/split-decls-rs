use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AsRef < [u8] > for Mmap { fn as_ref (& self) -> & [u8] { & self . 0 } }
}