use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Default)] pub (crate) struct SpanInterner { spans : FxIndexSet < SpanData > , }