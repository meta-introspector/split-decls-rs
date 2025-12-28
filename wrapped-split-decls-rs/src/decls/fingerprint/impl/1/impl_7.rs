use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FingerprintComponent for Hash64 { # [inline] fn as_u64 (& self) -> u64 { Hash64 :: as_u64 (* self) } }