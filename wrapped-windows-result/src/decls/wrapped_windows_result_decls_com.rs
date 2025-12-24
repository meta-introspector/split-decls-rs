use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(windows, not(windows_slim_errors)))]
mod com;
