use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : SpanDecoder > Decodable < D > for Symbol { fn decode (s : & mut D) -> Symbol { s . decode_symbol () } }