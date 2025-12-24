use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(any(windows, target_os = "espidf")))]
#[cfg(any(feature = "thread", feature = "time"))]
mod clockid;
