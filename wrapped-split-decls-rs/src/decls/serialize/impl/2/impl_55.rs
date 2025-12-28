use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder , T : Decodable < D > + Copy > Decodable < D > for Cell < T > { fn decode (d : & mut D) -> Cell < T > { Cell :: new (Decodable :: decode (d)) } }
}