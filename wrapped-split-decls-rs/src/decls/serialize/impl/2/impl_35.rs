use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Encoder , T : Encodable < S > > Encodable < S > for [T] { default fn encode (& self , s : & mut S) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }