use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < & ffi :: CStr > for SmallCStr { fn from (s : & ffi :: CStr) -> Self { Self { data : SmallVec :: from_slice (s . to_bytes_with_nul ()) } } }