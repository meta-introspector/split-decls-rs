// Generated macro for panic_propagate_a (function)
macro_rules! Depcrate_join_testspanic_propagate_a {
() => {
// Module: crate::join::tests
// Provides: {"panic_propagate_a"}
// Dependencies: {}
# [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_a () { join (| | panic ! ("Hello, world!") , | | ()) ; }
};
}
