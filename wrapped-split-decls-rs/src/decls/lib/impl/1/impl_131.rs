use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Encoder > Encodable < S > for BytePos { fn encode (& self , s : & mut S) { s . emit_u32 (self . 0) ; } }