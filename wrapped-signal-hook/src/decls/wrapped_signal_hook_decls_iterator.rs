use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(not(windows), feature = "iterator"))]
#[cfg_attr(docsrs, doc(cfg(all(not(windows), feature = "iterator"))))]
pub mod iterator;
