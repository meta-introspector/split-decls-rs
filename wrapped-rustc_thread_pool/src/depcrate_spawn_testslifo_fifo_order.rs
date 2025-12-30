// Generated macro for lifo_fifo_order (function)
macro_rules! Depcrate_spawn_testslifo_fifo_order {
() => {
// Module: crate::spawn::tests
// Provides: {"lifo_fifo_order"}
// Dependencies: {}
# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn lifo_fifo_order () { let vec = test_order ! (spawn , spawn_fifo) ; let expected : Vec < i32 > = (0 .. 10) . rev () . flat_map (| i | (0 .. 10) . map (move | j | i * 10 + j)) . collect () ; assert_eq ! (vec , expected) ; }
};
}
