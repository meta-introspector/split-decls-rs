use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MaybeTempDir { pub fn new (dir : TempDir , keep_on_drop : bool) -> MaybeTempDir { MaybeTempDir { dir : ManuallyDrop :: new (dir) , keep : keep_on_drop } } }
}