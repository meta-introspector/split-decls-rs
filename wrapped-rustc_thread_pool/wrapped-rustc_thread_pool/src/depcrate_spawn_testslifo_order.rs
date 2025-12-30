// Generated macro for lifo_order (function)
macro_rules! Depcrate_spawn_testslifo_order {
() => {
// Module: crate::spawn::tests
// Provides: {"lifo_order"}
// Dependencies: {}
# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn lifo_order () { let vec = test_order ! (spawn , spawn) ; let expected : Vec < i32 > = (0 .. 100) . rev () . collect () ; assert_eq ! (vec , expected) ; }
};
}
