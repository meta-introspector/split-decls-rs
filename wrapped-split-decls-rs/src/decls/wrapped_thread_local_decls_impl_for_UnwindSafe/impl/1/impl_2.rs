use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Send + UnwindSafe > UnwindSafe for ThreadLocal < T > { }