use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < [u8 ; OUT_LEN] > for Hash { # [inline] fn from (bytes : [u8 ; OUT_LEN]) -> Self { Self :: from_bytes (bytes) } }