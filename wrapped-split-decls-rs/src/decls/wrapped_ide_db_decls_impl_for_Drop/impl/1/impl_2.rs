use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Drop for RootDatabase { fn drop (& mut self) { unsafe { ManuallyDrop :: drop (& mut self . storage) } ; } }