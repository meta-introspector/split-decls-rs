use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < E : Encoder , K , V , S > Encodable < E > for HashMap < K , V , S > where K : Encodable < E > + Eq , V : Encodable < E > , S : BuildHasher , { fn encode (& self , e : & mut E) { e . emit_usize (self . len ()) ; for (key , val) in self { key . encode (e) ; val . encode (e) ; } } }
}