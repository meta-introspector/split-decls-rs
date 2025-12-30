// Generated macro for impl_169 (impl)
macro_rules! Depcrate_testsimpl_169 {
() => {
// Module: crate::tests
// Provides: {"impl_169"}
// Dependencies: {}
impl < 'a > Drop for DropTest < 'a > { fn drop (& mut self) { self . counter . fetch_sub (1 , Ordering :: Relaxed) ; } }
};
}
