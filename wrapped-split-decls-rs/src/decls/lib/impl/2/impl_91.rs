use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : SpanDecoder > Decodable < D > for ByteSymbol { fn decode (s : & mut D) -> ByteSymbol { s . decode_byte_symbol () } }
}