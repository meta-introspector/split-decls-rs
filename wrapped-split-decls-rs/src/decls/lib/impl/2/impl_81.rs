use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < E : SpanEncoder > Encodable < E > for ExpnId { fn encode (& self , s : & mut E) { s . encode_expn_id (* self) } }
}