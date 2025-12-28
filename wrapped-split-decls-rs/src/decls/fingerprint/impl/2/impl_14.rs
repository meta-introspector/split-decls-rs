use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FingerprintHasher for crate :: unhash :: Unhasher { # [inline] fn write_fingerprint (& mut self , fingerprint : & Fingerprint) { self . write_u64 (fingerprint . 0 . wrapping_add (fingerprint . 1)) ; } }
}