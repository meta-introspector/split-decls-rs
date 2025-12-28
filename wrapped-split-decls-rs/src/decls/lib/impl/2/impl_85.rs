use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < E : SpanEncoder > Encodable < E > for DefId { fn encode (& self , s : & mut E) { s . encode_def_id (* self) } }
}