// Generated macro for panic_propagate_nested_spawn (function)
macro_rules! Depcrate_scope_testspanic_propagate_nested_spawn {
() => {
// Module: crate::scope::tests
// Provides: {"panic_propagate_nested_spawn"}
// Dependencies: {}
# [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_nested_spawn () { scope (| s | s . spawn (| s | s . spawn (| s | s . spawn (| _ | panic ! ("Hello, world!"))))) ; }
};
}
