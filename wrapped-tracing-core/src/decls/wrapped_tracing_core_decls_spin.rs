use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "std"))]
pub(crate) mod spin;
