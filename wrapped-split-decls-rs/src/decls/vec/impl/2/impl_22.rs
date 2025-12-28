use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "nightly")] impl < S : Encoder , I : Idx , T : Encodable < S > > Encodable < S > for IndexVec < I , T > { fn encode (& self , s : & mut S) { Encodable :: encode (& self . raw , s) ; } }