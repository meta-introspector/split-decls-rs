use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : Encoder > Encodable < S > for Hash64 { # [inline] fn encode (& self , s : & mut S) { s . emit_raw_bytes (& self . as_u64 () . to_le_bytes ()) ; } }
}