use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "xxhash64")]
#[cfg_attr(docsrs, doc(cfg(feature = "xxhash64")))]
pub mod xxhash64;
