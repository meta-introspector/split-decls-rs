// Generated macro for waker_current_thread_id (function)
macro_rules! Depcrate_sync_mpmc_testswaker_current_thread_id {
() => {
// Module: crate::sync::mpmc::tests
// Provides: {"waker_current_thread_id"}
// Dependencies: {}
# [test] fn waker_current_thread_id () { let first = super :: waker :: current_thread_id () ; let t = crate :: thread :: spawn (move | | { let second = super :: waker :: current_thread_id () ; assert_ne ! (first , second) ; assert_eq ! (second , super :: waker :: current_thread_id ()) ; }) ; assert_eq ! (first , super :: waker :: current_thread_id ()) ; t . join () . unwrap () ; assert_eq ! (first , super :: waker :: current_thread_id ()) ; }
};
}
