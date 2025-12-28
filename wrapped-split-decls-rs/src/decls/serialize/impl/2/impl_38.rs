use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : Encoder , T : Encodable < S > , const N : usize > Encodable < S > for [T ; N] { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
}