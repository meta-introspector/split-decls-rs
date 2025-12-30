// Generated macro for panic_propagate_nested_scope_spawn (function)
macro_rules! Depcrate_scope_testspanic_propagate_nested_scope_spawn {
() => {
// Module: crate::scope::tests
// Provides: {"panic_propagate_nested_scope_spawn"}
// Dependencies: {}
# [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_nested_scope_spawn () { scope (| s | s . spawn (| _ | scope (| s | s . spawn (| _ | panic ! ("Hello, world!"))))) ; }
};
}
