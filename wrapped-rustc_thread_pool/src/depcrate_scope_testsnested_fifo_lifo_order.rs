// Generated macro for nested_fifo_lifo_order (function)
macro_rules! Depcrate_scope_testsnested_fifo_lifo_order {
() => {
// Module: crate::scope::tests
// Provides: {"nested_fifo_lifo_order"}
// Dependencies: {}
# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn nested_fifo_lifo_order () { let vec = test_nested_order ! (scope_fifo => spawn_fifo , scope => spawn) ; let expected : Vec < i32 > = (0 .. 10) . flat_map (| i | (0 .. 10) . rev () . map (move | j | i * 10 + j)) . collect () ; assert_eq ! (vec , expected) ; }
};
}
