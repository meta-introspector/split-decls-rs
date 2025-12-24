use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(target_os = "wasi"))]
#[cfg(feature = "net")]
#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
pub mod net;
