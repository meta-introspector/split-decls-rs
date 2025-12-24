use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "xxhash32")]
#[cfg_attr(docsrs, doc(cfg(feature = "xxhash32")))]
pub mod xxhash32;
