use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder > Decodable < D > for Cow < '_ , str > { fn decode (d : & mut D) -> Cow < 'static , str > { let v : String = Decodable :: decode (d) ; Cow :: Owned (v) } }
}