use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : Encoder > Encodable < S > for () { fn encode (& self , _s : & mut S) { } }
}