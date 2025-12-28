use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
trait FingerprintHasher { fn write_fingerprint (& mut self , fingerprint : & Fingerprint) ; }
}