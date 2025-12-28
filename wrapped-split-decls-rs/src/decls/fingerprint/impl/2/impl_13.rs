use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < H : Hasher > FingerprintHasher for H { # [inline] default fn write_fingerprint (& mut self , fingerprint : & Fingerprint) { self . write_u64 (fingerprint . 0) ; self . write_u64 (fingerprint . 1) ; } }
}