use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > Decodable < MemDecoder < 'a > > for Vec < u8 > { fn decode (d : & mut MemDecoder < 'a >) -> Self { let len = Decoder :: read_usize (d) ; d . read_raw_bytes (len) . to_owned () } }
}