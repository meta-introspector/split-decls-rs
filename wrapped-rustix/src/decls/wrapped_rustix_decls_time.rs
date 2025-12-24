use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(any(windows, target_os = "espidf")))]
#[cfg(feature = "time")]
#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
pub mod time;
