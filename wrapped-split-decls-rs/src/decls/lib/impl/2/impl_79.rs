use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < E : SpanEncoder > Encodable < E > for Symbol { fn encode (& self , s : & mut E) { s . encode_symbol (* self) ; } }
}