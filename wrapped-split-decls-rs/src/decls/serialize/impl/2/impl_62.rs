use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Encoder , A : Array < Item : Encodable < S > > > Encodable < S > for SmallVec < A > { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }