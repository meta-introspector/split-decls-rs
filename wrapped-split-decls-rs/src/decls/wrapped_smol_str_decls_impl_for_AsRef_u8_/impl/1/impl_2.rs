use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl AsRef < [u8] > for SmolStr { # [inline (always)] fn as_ref (& self) -> & [u8] { self . as_str () . as_bytes () } }