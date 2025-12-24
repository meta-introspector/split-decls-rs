use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "system")]
#[cfg(not(any(windows, target_os = "wasi")))]
#[cfg_attr(docsrs, doc(cfg(feature = "system")))]
pub mod system;
