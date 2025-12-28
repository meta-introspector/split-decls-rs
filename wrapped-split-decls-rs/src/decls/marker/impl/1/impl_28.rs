use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : DynSync > Sync for FromDyn < T > { }