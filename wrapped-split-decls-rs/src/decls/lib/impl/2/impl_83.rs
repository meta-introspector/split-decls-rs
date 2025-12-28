use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < E : SpanEncoder > Encodable < E > for CrateNum { fn encode (& self , s : & mut E) { s . encode_crate_num (* self) } }
}