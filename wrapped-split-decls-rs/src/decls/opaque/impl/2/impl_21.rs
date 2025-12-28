use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Encodable < FileEncoder > for [u8] { fn encode (& self , e : & mut FileEncoder) { Encoder :: emit_usize (e , self . len ()) ; e . emit_raw_bytes (self) ; } }
}