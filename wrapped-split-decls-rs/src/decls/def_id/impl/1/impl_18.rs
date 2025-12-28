use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Default for DefPathHash { fn default () -> Self { DefPathHash (Fingerprint :: ZERO) } }