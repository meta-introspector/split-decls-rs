use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > Drop for IoUring < S , C > { fn drop (& mut self) { unsafe { ManuallyDrop :: drop (& mut self . memory) ; } } }
}