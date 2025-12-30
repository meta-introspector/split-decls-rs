// Generated macro for impl_192 (impl)
macro_rules! Depcrate_stackimpl_192 {
() => {
// Module: crate::stack
// Provides: {"impl_192"}
// Dependencies: {}
impl < T > Drop for Stack < T > { # [inline] fn drop (& mut self) { if ! self . newest . is_null (Relaxed) { let guard = Guard :: new () ; let mut iter = self . iter (& guard) ; while let Some (entry) = iter . current . as_ref () { entry . delete_self (Relaxed) ; iter . next () ; } } } }
};
}
