// Generated macro for nested_lifo_order (function)
macro_rules! Depcrate_scope_testsnested_lifo_order {
() => {
// Module: crate::scope::tests
// Provides: {"nested_lifo_order"}
// Dependencies: {}
# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn nested_lifo_order () { let vec = test_nested_order ! (scope => spawn , scope => spawn) ; let expected : Vec < i32 > = (0 .. 100) . rev () . collect () ; assert_eq ! (vec , expected) ; }
};
}
