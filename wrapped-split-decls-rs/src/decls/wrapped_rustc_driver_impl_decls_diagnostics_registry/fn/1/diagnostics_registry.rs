use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn diagnostics_registry () -> Registry { Registry :: new (rustc_errors :: codes :: DIAGNOSTICS) }