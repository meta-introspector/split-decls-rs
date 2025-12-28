use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < E : Encoder > Encodable < E > for PackedFingerprint { # [inline] fn encode (& self , s : & mut E) { let copy = self . 0 ; copy . encode (s) ; } }
}