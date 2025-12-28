use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : Sync > Sync for ThinVec < T > { }