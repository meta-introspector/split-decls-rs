use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Encodable < MemEncoder > for [u8] { fn encode (& self , e : & mut MemEncoder) { Encoder :: emit_usize (e , self . len ()) ; e . emit_raw_bytes (self) ; } }