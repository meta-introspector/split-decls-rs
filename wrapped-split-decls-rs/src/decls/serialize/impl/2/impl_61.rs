use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder , T : Decodable < D > > Decodable < D > for Box < T > { fn decode (d : & mut D) -> Box < T > { Box :: new (Decodable :: decode (d)) } }
}