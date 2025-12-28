use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Drop for DropCounter < '_ > { fn drop (& mut self) { self . count . set (self . count . get () + 1) ; } }