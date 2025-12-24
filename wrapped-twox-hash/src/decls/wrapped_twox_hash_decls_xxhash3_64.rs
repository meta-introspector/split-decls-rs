use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "xxhash3_64")]
#[cfg_attr(docsrs, doc(cfg(feature = "xxhash3_64")))]
pub mod xxhash3_64;
