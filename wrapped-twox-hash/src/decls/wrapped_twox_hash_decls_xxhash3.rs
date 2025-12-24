use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(any(feature = "xxhash3_64", feature = "xxhash3_128"))]
mod xxhash3;
