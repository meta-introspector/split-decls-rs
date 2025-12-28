use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : SpanDecoder > Decodable < D > for DefIndex { fn decode (s : & mut D) -> DefIndex { s . decode_def_index () } }
}