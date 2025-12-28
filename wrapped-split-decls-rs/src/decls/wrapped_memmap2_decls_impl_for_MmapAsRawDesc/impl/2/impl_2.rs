use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (windows)] impl < T > MmapAsRawDesc for & T where T : AsRawHandle , { fn as_raw_desc (& self) -> MmapRawDescriptor { MmapRawDescriptor (self . as_raw_handle ()) } }