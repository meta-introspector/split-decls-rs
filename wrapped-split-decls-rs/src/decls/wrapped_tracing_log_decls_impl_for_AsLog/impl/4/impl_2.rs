use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AsLog for tracing_core :: LevelFilter { type Log = log :: LevelFilter ; # [inline] fn as_log (& self) -> Self :: Log { match * self { tracing_core :: LevelFilter :: OFF => log :: LevelFilter :: Off , tracing_core :: LevelFilter :: ERROR => log :: LevelFilter :: Error , tracing_core :: LevelFilter :: WARN => log :: LevelFilter :: Warn , tracing_core :: LevelFilter :: INFO => log :: LevelFilter :: Info , tracing_core :: LevelFilter :: DEBUG => log :: LevelFilter :: Debug , tracing_core :: LevelFilter :: TRACE => log :: LevelFilter :: Trace , } } }
}