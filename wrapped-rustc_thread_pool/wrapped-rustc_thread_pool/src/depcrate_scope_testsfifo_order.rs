// Generated macro for fifo_order (function)
macro_rules! Depcrate_scope_testsfifo_order {
() => {
// Module: crate::scope::tests
// Provides: {"fifo_order"}
// Dependencies: {}
# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn fifo_order () { let vec = test_order ! (scope_fifo => spawn_fifo) ; let expected : Vec < i32 > = (0 .. 100) . collect () ; assert_eq ! (vec , expected) ; }
};
}
