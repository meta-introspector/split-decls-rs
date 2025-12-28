use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < DummySpan > for Span { fn from (_ : DummySpan) -> Self { rustc_span :: DUMMY_SP } }
}