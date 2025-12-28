use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Encoder , T : Encodable < S > > Encodable < S > for Cow < '_ , [T] > where [T] : ToOwned < Owned = Vec < T > > , { fn encode (& self , s : & mut S) { let slice : & [T] = self ; slice . encode (s) ; } }