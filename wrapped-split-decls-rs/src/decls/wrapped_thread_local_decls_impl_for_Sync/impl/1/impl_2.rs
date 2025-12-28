use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : Send > Sync for ThreadLocal < T > { }