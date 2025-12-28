use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FileId { const MAX : u32 = 0x7fff_ffff ; # [inline] pub const fn from_raw (raw : u32) -> FileId { assert ! (raw <= Self :: MAX) ; FileId (raw) } # [inline] pub const fn index (self) -> u32 { self . 0 } }