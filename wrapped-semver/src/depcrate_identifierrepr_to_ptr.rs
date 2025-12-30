// Generated macro for repr_to_ptr (function)
macro_rules! Depcrate_identifierrepr_to_ptr {
() => {
// Module: crate::identifier
// Provides: {"repr_to_ptr"}
// Dependencies: {}
fn repr_to_ptr (modified : NonNull < u8 >) -> * const u8 { let modified = modified . as_ptr () ; let original = (modified as usize) << 1 ; let diff = original . wrapping_sub (modified as usize) ; modified . wrapping_add (diff) }
};
}
