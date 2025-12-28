use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < u32 > for RawIdx { # [inline] fn from (idx : u32) -> RawIdx { RawIdx (idx) } }
}