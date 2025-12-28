use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : SpanDecoder > Decodable < D > for LocalDefId { fn decode (d : & mut D) -> LocalDefId { DefId :: decode (d) . expect_local () } }