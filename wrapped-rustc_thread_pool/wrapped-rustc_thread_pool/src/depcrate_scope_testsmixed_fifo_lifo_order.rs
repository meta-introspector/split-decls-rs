// Generated macro for mixed_fifo_lifo_order (function)
macro_rules! Depcrate_scope_testsmixed_fifo_lifo_order {
() => {
// Module: crate::scope::tests
// Provides: {"mixed_fifo_lifo_order"}
// Dependencies: {}
# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn mixed_fifo_lifo_order () { let vec = test_mixed_order ! (scope_fifo => spawn_fifo , scope => spawn) ; let expected = vec ! [- 3 , 0 , - 2 , 1 , - 1 , 2 , 3] ; assert_eq ! (vec , expected) ; }
};
}
