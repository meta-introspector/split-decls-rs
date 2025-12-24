use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(any(windows, target_os = "espidf")))]
#[cfg(feature = "param")]
#[cfg_attr(docsrs, doc(cfg(feature = "param")))]
pub mod param;
