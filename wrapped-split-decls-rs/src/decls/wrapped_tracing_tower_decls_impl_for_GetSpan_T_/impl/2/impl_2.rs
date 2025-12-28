use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > GetSpan < T > for tracing :: Span { # [inline] fn span_for (& self , _ : & T) -> tracing :: Span { self . clone () } }
}