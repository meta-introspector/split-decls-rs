use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : SpanDecoder > Decodable < D > for Symbol { fn decode (s : & mut D) -> Symbol { s . decode_symbol () } }
}