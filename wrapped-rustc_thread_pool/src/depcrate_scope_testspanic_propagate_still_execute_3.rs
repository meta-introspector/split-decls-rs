// Generated macro for panic_propagate_still_execute_3 (function)
macro_rules! Depcrate_scope_testspanic_propagate_still_execute_3 {
() => {
// Module: crate::scope::tests
// Provides: {"panic_propagate_still_execute_3"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn panic_propagate_still_execute_3 () { let mut x = false ; let result = unwind :: halt_unwinding (| | { scope (| s | { s . spawn (| _ | x = true) ; panic ! ("Hello, world!") ; }) ; }) ; match result { Ok (_) => panic ! ("failed to propagate panic") , Err (_) => assert ! (x , "panic after spawn, spawn failed to execute") , } }
};
}
