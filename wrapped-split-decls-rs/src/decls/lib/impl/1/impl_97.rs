use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : SpanDecoder > Decodable < D > for AttrId { fn decode (s : & mut D) -> AttrId { s . decode_attr_id () } }