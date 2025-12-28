use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl AsRef < [u8] > for MmapMut { # [inline] fn as_ref (& self) -> & [u8] { self . deref () } }