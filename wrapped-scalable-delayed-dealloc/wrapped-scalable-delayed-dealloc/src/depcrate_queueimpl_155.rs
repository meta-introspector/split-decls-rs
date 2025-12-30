// Generated macro for impl_155 (impl)
macro_rules! Depcrate_queueimpl_155 {
() => {
// Module: crate::queue
// Provides: {"impl_155"}
// Dependencies: {}
impl < T > Drop for Queue < T > { # [inline] fn drop (& mut self) { if ! self . oldest . is_null (Relaxed) { let guard = Guard :: new () ; let mut iter = self . iter (& guard) ; while let Some (entry) = iter . current . as_ref () { entry . delete_self (Relaxed) ; iter . next () ; } } } }
};
}
