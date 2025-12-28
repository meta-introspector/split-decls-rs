use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : Encoder > Encodable < S > for Pu128 { # [inline] fn encode (& self , s : & mut S) { { self . 0 } . encode (s) ; } }
}