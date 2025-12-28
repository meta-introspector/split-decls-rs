use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Span > for MultiSpan { fn from (span : Span) -> MultiSpan { MultiSpan :: from_span (span) } }
}