use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl AsRef < [u8] > for Uuid { # [inline] fn as_ref (& self) -> & [u8] { & self . 0 } }