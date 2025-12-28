use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Default)] pub (crate) struct SpanInterner { spans : FxIndexSet < SpanData > , }
}