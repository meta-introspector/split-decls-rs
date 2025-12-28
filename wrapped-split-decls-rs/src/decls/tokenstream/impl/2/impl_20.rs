use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : SpanEncoder > Encodable < S > for LazyAttrTokenStream { fn encode (& self , _s : & mut S) { panic ! ("Attempted to encode LazyAttrTokenStream") ; } }
}