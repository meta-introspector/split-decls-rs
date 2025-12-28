use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : Sync > Send for ScopePtr < T > { }