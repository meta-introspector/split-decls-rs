// Generated macro for mixed_lifo_fifo_order (function)
macro_rules! Depcrate_scope_testsmixed_lifo_fifo_order {
() => {
// Module: crate::scope::tests
// Provides: {"mixed_lifo_fifo_order"}
// Dependencies: {}
# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn mixed_lifo_fifo_order () { let vec = test_mixed_order ! (scope => spawn , scope_fifo => spawn_fifo) ; let expected = vec ! [- 1 , 2 , - 2 , 1 , - 3 , 3 , 0] ; assert_eq ! (vec , expected) ; }
};
}
