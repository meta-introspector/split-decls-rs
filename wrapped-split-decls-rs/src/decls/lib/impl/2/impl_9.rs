use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < # [may_dangle] T > Drop for ArenaChunk < T > { fn drop (& mut self) { unsafe { drop (Box :: from_raw (self . storage . as_mut ())) } } }