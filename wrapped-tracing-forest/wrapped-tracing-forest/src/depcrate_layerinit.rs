// Generated macro for init (function)
macro_rules! Depcrate_layerinit {
() => {
// Module: crate::layer
// Provides: {"init"}
// Dependencies: {}
# [doc = " Initializes a global subscriber with a [`ForestLayer`] using the default configuration."] # [doc = ""] # [doc = " This function is intended for quick initialization and processes log trees \"inline\","] # [doc = " meaning it doesn't take advantage of a worker task for formatting and writing."] # [doc = " To use a worker task, consider using the [`worker_task`] function. Alternatively,"] # [doc = " configure a `Subscriber` manually using a `ForestLayer`."] # [doc = ""] # [doc = " [`worker_task`]: crate::runtime::worker_task"] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use tracing::{info, info_span};"] # [doc = ""] # [doc = " tracing_forest::init();"] # [doc = ""] # [doc = " info!(\"Hello, world!\");"] # [doc = " info_span!(\"my_span\").in_scope(|| {"] # [doc = "     info!(\"Relevant information\");"] # [doc = " });"] # [doc = " ```"] # [doc = " Produces the the output:"] # [doc = " ```log"] # [doc = " INFO     ｉ [info]: Hello, world!"] # [doc = " INFO     my_span [ 26.0µs | 100.000% ]"] # [doc = " INFO     ┕━ ｉ [info]: Relevant information"] # [doc = " ```"] pub fn init () { Registry :: default () . with (ForestLayer :: default ()) . init () ; }
};
}
