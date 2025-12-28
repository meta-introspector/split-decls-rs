use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < E : Encoder > Encodable < E > for Fingerprint { # [inline] fn encode (& self , s : & mut E) { s . emit_raw_bytes (& self . to_le_bytes ()) ; } }
}