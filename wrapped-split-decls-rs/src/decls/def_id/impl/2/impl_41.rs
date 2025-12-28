use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < E : SpanEncoder > Encodable < E > for LocalDefId { fn encode (& self , s : & mut E) { self . to_def_id () . encode (s) ; } }
}