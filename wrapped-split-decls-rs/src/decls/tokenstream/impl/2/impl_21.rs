use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : SpanDecoder > Decodable < D > for LazyAttrTokenStream { fn decode (_d : & mut D) -> Self { panic ! ("Attempted to decode LazyAttrTokenStream") ; } }
}