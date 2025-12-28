use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Encoder > Encodable < S > for str { fn encode (& self , s : & mut S) { s . emit_str (self) ; } }