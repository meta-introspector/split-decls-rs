use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : SpanDecoder > Decodable < D > for LocalExpnId { fn decode (d : & mut D) -> Self { ExpnId :: expect_local (ExpnId :: decode (d)) } }
}