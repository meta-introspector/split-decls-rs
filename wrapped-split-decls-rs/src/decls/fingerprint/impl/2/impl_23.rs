use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Fingerprint > for PackedFingerprint { # [inline] fn from (f : Fingerprint) -> PackedFingerprint { PackedFingerprint (f) } }
}