use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder , T : Decodable < D > > Decodable < D > for Option < T > { fn decode (d : & mut D) -> Option < T > { match d . read_u8 () { 0 => None , 1 => Some (Decodable :: decode (d)) , _ => panic ! ("Encountered invalid discriminant while decoding `Option`.") , } } }
}