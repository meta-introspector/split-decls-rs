// Generated macro for join_a_lot (function)
macro_rules! Depcrate_thread_pool_testsjoin_a_lot {
() => {
// Module: crate::thread_pool::tests
// Provides: {"join_a_lot"}
// Dependencies: {}
fn join_a_lot (n : usize) { if n > 0 { join (| | join_a_lot (n - 1) , | | join_a_lot (n - 1)) ; } }
};
}
