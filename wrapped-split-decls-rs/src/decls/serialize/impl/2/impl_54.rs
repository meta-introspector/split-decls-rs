use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Encoder , T : Encodable < S > + Copy > Encodable < S > for Cell < T > { fn encode (& self , s : & mut S) { self . get () . encode (s) ; } }