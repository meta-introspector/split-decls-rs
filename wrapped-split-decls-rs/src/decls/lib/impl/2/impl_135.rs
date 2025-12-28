use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder > Decodable < D > for RelativeBytePos { fn decode (d : & mut D) -> RelativeBytePos { RelativeBytePos (d . read_u32 ()) } }
}