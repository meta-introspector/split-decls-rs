// Generated macro for init_log (function)
macro_rules! Depcrate_testinit_log {
() => {
// Module: crate::test
// Provides: {"init_log"}
// Dependencies: {}
fn init_log () { let _ = tracing_subscriber :: fmt () . with_test_writer () . try_init () ; }
};
}
