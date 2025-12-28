use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : Encoder , T : Encodable < S > > Encodable < S > for Option < T > { fn encode (& self , s : & mut S) { match * self { None => s . emit_u8 (0) , Some (ref v) => { s . emit_u8 (1) ; v . encode (s) ; } } } }
}