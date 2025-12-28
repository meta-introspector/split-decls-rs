use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [cfg (windows)] pub struct MmapRawDescriptor (RawHandle) ;
}