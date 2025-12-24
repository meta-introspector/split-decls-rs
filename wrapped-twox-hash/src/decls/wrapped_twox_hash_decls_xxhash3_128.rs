use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "xxhash3_128")]
#[cfg_attr(docsrs, doc(cfg(feature = "xxhash3_128")))]
pub mod xxhash3_128;
