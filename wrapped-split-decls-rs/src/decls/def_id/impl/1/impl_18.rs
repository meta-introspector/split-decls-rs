use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for DefPathHash { fn default () -> Self { DefPathHash (Fingerprint :: ZERO) } }
}