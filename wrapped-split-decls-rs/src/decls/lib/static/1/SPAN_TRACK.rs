use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static SPAN_TRACK : AtomicRef < fn (LocalDefId) > = AtomicRef :: new (& ((| _ | { }) as fn (_))) ;