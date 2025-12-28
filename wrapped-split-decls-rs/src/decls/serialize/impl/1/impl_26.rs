use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : Encoder > Encodable < S > for String { fn encode (& self , s : & mut S) { s . emit_str (& self) ; } }
}