// Generated macro for mixed_fifo_order (function)
macro_rules! Depcrate_scope_testsmixed_fifo_order {
() => {
// Module: crate::scope::tests
// Provides: {"mixed_fifo_order"}
// Dependencies: {}
# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn mixed_fifo_order () { let vec = test_mixed_order ! (scope_fifo => spawn_fifo , scope_fifo => spawn_fifo) ; let expected = vec ! [- 1 , 0 , - 2 , 1 , - 3 , 2 , 3] ; assert_eq ! (vec , expected) ; }
};
}
