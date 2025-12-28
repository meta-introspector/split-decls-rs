use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Drop for JodChild { fn drop (& mut self) { _ = self . 0 . kill () ; _ = self . 0 . wait () ; } }