use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : SpanDecoder > Decodable < D > for ExpnId { fn decode (s : & mut D) -> ExpnId { s . decode_expn_id () } }
}