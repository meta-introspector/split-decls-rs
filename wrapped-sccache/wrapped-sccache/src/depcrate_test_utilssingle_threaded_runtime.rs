// Generated macro for single_threaded_runtime (function)
macro_rules! Depcrate_test_utilssingle_threaded_runtime {
() => {
// Module: crate::test::utils
// Provides: {"single_threaded_runtime"}
// Dependencies: {}
pub fn single_threaded_runtime () -> tokio :: runtime :: Runtime { tokio :: runtime :: Builder :: new_current_thread () . enable_all () . worker_threads (1) . build () . unwrap () }
};
}
