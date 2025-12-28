use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < BS : ArraySize , K : BufferKind > Drop for BlockBuffer < BS , K > { # [inline] fn drop (& mut self) { # [cfg (feature = "zeroize")] self . zeroize () ; } }
}