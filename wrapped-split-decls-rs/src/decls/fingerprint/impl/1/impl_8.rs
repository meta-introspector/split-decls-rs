use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FingerprintComponent for u64 { # [inline] fn as_u64 (& self) -> u64 { * self } }
}