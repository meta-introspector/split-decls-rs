use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder , T : Decodable < D > > Decodable < D > for Rc < T > { fn decode (d : & mut D) -> Rc < T > { Rc :: new (Decodable :: decode (d)) } }
}