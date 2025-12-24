use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "fuzz_exports"))]
pub(crate) mod huff0;
