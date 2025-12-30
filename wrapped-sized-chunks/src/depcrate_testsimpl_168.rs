// Generated macro for impl_168 (impl)
macro_rules! Depcrate_testsimpl_168 {
() => {
// Module: crate::tests
// Provides: {"impl_168"}
// Dependencies: {}
impl < 'a > DropTest < 'a > { pub (crate) fn new (counter : & 'a AtomicUsize) -> Self { counter . fetch_add (1 , Ordering :: Relaxed) ; DropTest { counter } } }
};
}
