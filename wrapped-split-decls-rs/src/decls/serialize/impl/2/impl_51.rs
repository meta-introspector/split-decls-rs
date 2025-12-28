use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Encoder > Encodable < S > for path :: Path { fn encode (& self , e : & mut S) { self . to_str () . unwrap () . encode (e) ; } }