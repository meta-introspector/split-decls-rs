// Generated macro for test_init (function)
macro_rules! Depcrate_layertest_init {
() => {
// Module: crate::layer
// Provides: {"test_init"}
// Dependencies: {}
# [doc = " Initializes a global subscriber for cargo tests with a [`ForestLayer`] using the default"] # [doc = " configuration."] # [doc = ""] # [doc = " This function is intended for test case initialization and processes log trees \"inline\","] # [doc = " meaning it doesn't take advantage of a worker task for formatting and writing."] # [doc = ""] # [doc = " [`worker_task`]: crate::runtime::worker_task"] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use tracing::{info, info_span};"] # [doc = ""] # [doc = " let _ = tracing_forest::test_init();"] # [doc = ""] # [doc = " info!(\"Hello, world!\");"] # [doc = " info_span!(\"my_span\").in_scope(|| {"] # [doc = "     info!(\"Relevant information\");"] # [doc = " });"] # [doc = " ```"] pub fn test_init () -> Result < () , TryInitError > { Registry :: default () . with (ForestLayer :: new (TestCapturePrinter :: new () , NoTag)) . try_init () }
};
}
