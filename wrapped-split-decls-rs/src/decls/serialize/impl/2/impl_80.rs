use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < E : Encoder , T : Encodable < E > > Encodable < E > for Rc < [T] > { fn encode (& self , s : & mut E) { let slice : & [T] = self ; slice . encode (s) ; } }
}