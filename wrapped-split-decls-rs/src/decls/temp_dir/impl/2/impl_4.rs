use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Drop for MaybeTempDir { fn drop (& mut self) { let dir = unsafe { ManuallyDrop :: take (& mut self . dir) } ; if self . keep { let _ = dir . keep () ; } } }
}