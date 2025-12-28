use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : SpanDecoder > Decodable < D > for Span { fn decode (s : & mut D) -> Span { s . decode_span () } }
}