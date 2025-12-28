use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : SpanDecoder > Decodable < D > for SyntaxContext { fn decode (s : & mut D) -> SyntaxContext { s . decode_syntax_context () } }