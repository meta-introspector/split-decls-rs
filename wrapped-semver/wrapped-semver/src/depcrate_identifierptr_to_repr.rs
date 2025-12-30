// Generated macro for ptr_to_repr (function)
macro_rules! Depcrate_identifierptr_to_repr {
() => {
// Module: crate::identifier
// Provides: {"ptr_to_repr"}
// Dependencies: {}
fn ptr_to_repr (original : * mut u8) -> NonNull < u8 > { let modified = (original as usize | 1) . rotate_right (1) ; let diff = modified . wrapping_sub (original as usize) ; let modified = original . wrapping_add (diff) ; unsafe { NonNull :: new_unchecked (modified) } }
};
}
