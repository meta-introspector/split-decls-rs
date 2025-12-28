use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Encoder > Encodable < S > for Hash128 { # [inline] fn encode (& self , s : & mut S) { s . emit_raw_bytes (& self . as_u128 () . to_le_bytes ()) ; } }