use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn configure (_sess : & Session , _features : Option < & Features > , _path : & ast :: Path , _kind : & ast :: MetaItemKind , _span : rustc_span :: Span ,) -> bool { true }
}