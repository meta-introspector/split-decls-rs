use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : SpanDecoder > Decodable < D > for CrateNum { fn decode (s : & mut D) -> CrateNum { s . decode_crate_num () } }
}