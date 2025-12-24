use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(windows))]
#[cfg(feature = "thread")]
#[cfg_attr(docsrs, doc(cfg(feature = "thread")))]
pub mod thread;
