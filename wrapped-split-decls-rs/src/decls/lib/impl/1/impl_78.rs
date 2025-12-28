use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < E : SpanEncoder > Encodable < E > for Span { fn encode (& self , s : & mut E) { s . encode_span (* self) ; } }