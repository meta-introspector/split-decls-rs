use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < PackedFingerprint > for Fingerprint { # [inline] fn from (f : PackedFingerprint) -> Fingerprint { f . 0 } }
}