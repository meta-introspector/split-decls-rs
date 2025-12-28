use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait MmapAsRawDesc { fn as_raw_desc (& self) -> MmapRawDescriptor ; }
}