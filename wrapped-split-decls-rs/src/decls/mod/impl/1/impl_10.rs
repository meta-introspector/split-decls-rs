use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Drop for ThreadPool { fn drop (& mut self) { self . registry . terminate () ; } }