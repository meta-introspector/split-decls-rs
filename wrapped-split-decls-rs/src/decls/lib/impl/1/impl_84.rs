use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < E : SpanEncoder > Encodable < E > for DefIndex { fn encode (& self , s : & mut E) { s . encode_def_index (* self) } }