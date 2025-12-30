// Generated macro for test_join_panic (function)
macro_rules! Depcrate_thread_teststest_join_panic {
() => {
// Module: crate::thread::tests
// Provides: {"test_join_panic"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore = "test requires unwinding support")] fn test_join_panic () { match thread :: spawn (move | | panic ! ()) . join () { result :: Result :: Err (_) => () , result :: Result :: Ok (()) => panic ! () , } }
};
}
