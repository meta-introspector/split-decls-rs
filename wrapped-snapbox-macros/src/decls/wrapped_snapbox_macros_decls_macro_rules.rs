use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Feature-flag controlled additional test debug information
#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {};
}
