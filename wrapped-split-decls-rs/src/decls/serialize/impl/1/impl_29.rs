use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder > Decodable < D > for () { fn decode (_ : & mut D) { } }
}