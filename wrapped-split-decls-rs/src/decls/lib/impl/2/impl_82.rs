use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < E : SpanEncoder > Encodable < E > for SyntaxContext { fn encode (& self , s : & mut E) { s . encode_syntax_context (* self) } }