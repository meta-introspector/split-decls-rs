use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait FingerprintComponent { fn as_u64 (& self) -> u64 ; }