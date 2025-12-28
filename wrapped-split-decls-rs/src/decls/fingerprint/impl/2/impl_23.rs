use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < Fingerprint > for PackedFingerprint { # [inline] fn from (f : Fingerprint) -> PackedFingerprint { PackedFingerprint (f) } }