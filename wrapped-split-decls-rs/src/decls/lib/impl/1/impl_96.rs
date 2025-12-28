use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : SpanDecoder > Decodable < D > for DefId { fn decode (s : & mut D) -> DefId { s . decode_def_id () } }
}