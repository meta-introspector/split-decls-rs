use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder > Decodable < D > for BytePos { fn decode (d : & mut D) -> BytePos { BytePos (d . read_u32 ()) } }
}