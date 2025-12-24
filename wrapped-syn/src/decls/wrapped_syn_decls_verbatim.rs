use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(any(feature = "full", feature = "derive"), feature = "parsing"))]
mod verbatim;
