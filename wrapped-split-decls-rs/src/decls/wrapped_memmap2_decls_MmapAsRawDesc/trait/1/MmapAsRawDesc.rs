use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait MmapAsRawDesc { fn as_raw_desc (& self) -> MmapRawDescriptor ; }