use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(windows))]
#[macro_use]
pub(crate) mod cstr;
