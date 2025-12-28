use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : Encoder , T : Encodable < S > > Encodable < S > for Rc < T > { fn encode (& self , s : & mut S) { (* * self) . encode (s) ; } }
}