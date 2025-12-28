use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : Encoder , T : ? Sized + PointeeSized > Encodable < S > for & T where T : Encodable < S > , { fn encode (& self , s : & mut S) { (* * self) . encode (s) } }
}