use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < E : SpanEncoder > Encodable < E > for ByteSymbol { fn encode (& self , s : & mut E) { s . encode_byte_symbol (* self) ; } }