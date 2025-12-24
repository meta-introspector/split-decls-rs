use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(windows))]
#[cfg(feature = "process")]
#[cfg_attr(docsrs, doc(cfg(feature = "process")))]
pub mod process;
