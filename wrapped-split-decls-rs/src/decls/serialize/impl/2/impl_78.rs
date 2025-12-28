use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < E : Encoder , T , S > Encodable < E > for indexmap :: IndexSet < T , S > where T : Encodable < E > + Hash + Eq , S : BuildHasher , { fn encode (& self , s : & mut E) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }