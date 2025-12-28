use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Drop for SmallDroppable { fn drop (& mut self) { DROP_COUNTER . with (| c | c . set (c . get () + 1)) ; } }