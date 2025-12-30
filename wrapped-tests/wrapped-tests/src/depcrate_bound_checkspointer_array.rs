// Generated macro for pointer_array (function)
macro_rules! Depcrate_bound_checkspointer_array {
() => {
// Module: crate::bound_checks
// Provides: {"pointer_array"}
// Dependencies: {}
# [test] fn pointer_array () { let arr = NSPointerArray :: new () ; assert_throws ("attempt to access pointer at index 0 beyond bounds 0" , | | arr . pointerAtIndex (0) ,) ; }
};
}
