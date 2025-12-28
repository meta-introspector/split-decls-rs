use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : Encoder > Encodable < S > for Cow < '_ , str > { fn encode (& self , s : & mut S) { let val : & str = self ; val . encode (s) } }
}