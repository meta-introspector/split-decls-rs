use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < L : Sync > Sync for LatchRef < '_ , L > { }