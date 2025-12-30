// Generated macro for fifo_order (function)
macro_rules! Depcrate_spawn_testsfifo_order {
() => {
// Module: crate::spawn::tests
// Provides: {"fifo_order"}
// Dependencies: {}
# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn fifo_order () { let vec = test_order ! (spawn_fifo , spawn_fifo) ; let expected : Vec < i32 > = (0 .. 100) . collect () ; assert_eq ! (vec , expected) ; }
};
}
