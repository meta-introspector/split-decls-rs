use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Encoder , T : ? Sized + Encodable < S > > Encodable < S > for Box < T > { fn encode (& self , s : & mut S) { (* * self) . encode (s) } }