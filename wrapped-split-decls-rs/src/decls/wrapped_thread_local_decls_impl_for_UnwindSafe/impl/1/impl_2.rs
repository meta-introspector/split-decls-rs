use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Send + UnwindSafe > UnwindSafe for ThreadLocal < T > { }
}