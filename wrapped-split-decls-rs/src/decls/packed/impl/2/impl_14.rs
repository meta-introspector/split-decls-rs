use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder > Decodable < D > for Pu128 { # [inline] fn decode (d : & mut D) -> Self { Self (u128 :: decode (d)) } }
}