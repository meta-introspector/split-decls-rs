use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : SpanDecoder > Decodable < D > for AttrId { fn decode (s : & mut D) -> AttrId { s . decode_attr_id () } }
}