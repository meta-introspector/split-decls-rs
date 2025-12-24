use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(windows))]
#[cfg(not(target_os = "wasi"))]
#[cfg(feature = "pty")]
#[cfg_attr(docsrs, doc(cfg(feature = "pty")))]
pub mod pty;
