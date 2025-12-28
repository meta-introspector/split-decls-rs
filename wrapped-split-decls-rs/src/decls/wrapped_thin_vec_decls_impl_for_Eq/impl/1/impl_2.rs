use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > Eq for ThinVec < T > where T : Eq { }