use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < E : SpanEncoder > Encodable < E > for LocalExpnId { fn encode (& self , e : & mut E) { self . to_expn_id () . encode (e) ; } }
}