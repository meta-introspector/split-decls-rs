use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct SpanLowerer { is_incremental : bool , def_id : LocalDefId , }