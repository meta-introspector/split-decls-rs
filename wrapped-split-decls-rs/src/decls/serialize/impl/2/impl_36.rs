use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Encoder , T : Encodable < S > > Encodable < S > for Vec < T > { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }