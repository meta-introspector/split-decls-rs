// Generated macro for util (module)
macro_rules! Depcrateutil {
() => {
// Module: crate
// Provides: {"util"}
// Dependencies: {}
# [doc = " Bring Tracing's event and span macros into scope, along with other sensible defaults."] pub mod util { # [doc (no_inline)] pub use crate :: ForestLayer ; # [doc (no_inline)] pub use tracing :: metadata :: LevelFilter ; # [doc (no_inline)] pub use tracing :: { debug , debug_span , error , error_span , info , info_span , trace , trace_span , warn , warn_span , Event , Level , } ; # [cfg (feature = "env-filter")] # [doc (no_inline)] pub use tracing_subscriber :: EnvFilter ; }
};
}
