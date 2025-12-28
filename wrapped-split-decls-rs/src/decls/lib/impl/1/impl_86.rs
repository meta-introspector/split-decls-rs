use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < E : SpanEncoder > Encodable < E > for AttrId { fn encode (& self , _s : & mut E) { } }