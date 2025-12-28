use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < E : SpanEncoder > Encodable < E > for Symbol { fn encode (& self , s : & mut E) { s . encode_symbol (* self) ; } }