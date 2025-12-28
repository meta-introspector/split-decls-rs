use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct SpanLowerer { is_incremental : bool , def_id : LocalDefId , }
}