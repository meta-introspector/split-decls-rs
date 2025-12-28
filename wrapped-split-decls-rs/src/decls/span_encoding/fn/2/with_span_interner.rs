use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline] fn with_span_interner < T , F : FnOnce (& mut SpanInterner) -> T > (f : F) -> T { crate :: with_session_globals (| session_globals | f (& mut session_globals . span_interner . lock ())) }