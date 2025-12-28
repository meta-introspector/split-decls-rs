use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait FingerprintComponent { fn as_u64 (& self) -> u64 ; }
}