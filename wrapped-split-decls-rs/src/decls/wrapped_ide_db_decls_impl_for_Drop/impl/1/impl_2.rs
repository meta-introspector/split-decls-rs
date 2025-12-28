use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Drop for RootDatabase { fn drop (& mut self) { unsafe { ManuallyDrop :: drop (& mut self . storage) } ; } }
}