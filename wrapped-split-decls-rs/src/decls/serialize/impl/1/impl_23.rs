use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Encoder > Encodable < S > for NonZero < u32 > { fn encode (& self , s : & mut S) { s . emit_u32 (self . get ()) ; } }