use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub mod http;
