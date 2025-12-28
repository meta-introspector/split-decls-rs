use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Encoder > Encodable < S > for path :: PathBuf { fn encode (& self , e : & mut S) { path :: Path :: encode (self , e) ; } }