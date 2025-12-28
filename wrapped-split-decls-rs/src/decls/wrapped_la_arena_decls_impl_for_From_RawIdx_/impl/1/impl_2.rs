use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < RawIdx > for u32 { # [inline] fn from (raw : RawIdx) -> u32 { raw . 0 } }