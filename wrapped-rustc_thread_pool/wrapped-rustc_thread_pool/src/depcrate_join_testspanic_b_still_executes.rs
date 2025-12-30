// Generated macro for panic_b_still_executes (function)
macro_rules! Depcrate_join_testspanic_b_still_executes {
() => {
// Module: crate::join::tests
// Provides: {"panic_b_still_executes"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn panic_b_still_executes () { let mut x = false ; match unwind :: halt_unwinding (| | join (| | panic ! ("Hello, world!") , | | x = true)) { Ok (_) => panic ! ("failed to propagate panic from closure A,") , Err (_) => assert ! (x , "closure b failed to execute") , } }
};
}
