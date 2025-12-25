use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The predefined `HKEY_CURRENT_USER` registry key.
pub const CURRENT_USER: &Key = &Key(HKEY_CURRENT_USER);
