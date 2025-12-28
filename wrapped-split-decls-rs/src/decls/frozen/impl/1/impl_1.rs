use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > Frozen < T > { pub fn freeze (val : T) -> Self { Frozen (val) } }