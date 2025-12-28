use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < E : Encoder , T , S > Encodable < E > for HashSet < T , S > where T : Encodable < E > + Eq , S : BuildHasher , { fn encode (& self , s : & mut E) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
}