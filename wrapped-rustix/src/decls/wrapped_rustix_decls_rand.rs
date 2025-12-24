use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(windows))]
#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
pub mod rand;
