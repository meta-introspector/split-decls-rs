use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Export the rustc_internal APIs. Note that this module has no stability
/// guarantees and it is not taken into account for semver.
#[cfg(feature = "rustc_internal")]
pub mod rustc_internal;
