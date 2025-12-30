// Generated macro for impl_663 (impl)
macro_rules! Depcrate_util_and_thenimpl_663 {
() => {
// Module: crate::util::and_then
// Provides: {"impl_663"}
// Dependencies: {}
impl < F1 , F2 : TryFuture , N > AndThenFuture < F1 , F2 , N > { pub (crate) fn new (inner : future :: AndThen < future :: ErrInto < F1 , F2 :: Error > , F2 , N >) -> Self { Self { inner } } }
};
}
