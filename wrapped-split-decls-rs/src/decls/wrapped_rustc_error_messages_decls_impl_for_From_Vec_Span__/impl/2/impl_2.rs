use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Vec < Span > > for MultiSpan { fn from (spans : Vec < Span >) -> MultiSpan { MultiSpan :: from_spans (spans) } }
}