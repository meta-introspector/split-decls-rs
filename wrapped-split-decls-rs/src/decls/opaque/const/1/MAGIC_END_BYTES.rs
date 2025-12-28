use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const MAGIC_END_BYTES : & [u8] = b"rust-end-file" ;