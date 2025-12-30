// Generated macro for test_scoped_threads_nll (function)
macro_rules! Depcrate_thread_teststest_scoped_threads_nll {
() => {
// Module: crate::thread::tests
// Provides: {"test_scoped_threads_nll"}
// Dependencies: {}
# [test] fn test_scoped_threads_nll () { fn foo (x : & u8) { thread :: scope (| s | { s . spawn (| | match x { _ => () , }) ; }) ; } let x = 42_u8 ; foo (& x) ; }
};
}
