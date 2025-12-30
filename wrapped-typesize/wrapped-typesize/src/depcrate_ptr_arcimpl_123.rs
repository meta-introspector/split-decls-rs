// Generated macro for impl_123 (impl)
macro_rules! Depcrate_ptr_arcimpl_123 {
() => {
// Module: crate::ptr::arc
// Provides: {"impl_123"}
// Dependencies: {}
impl < T , SC : ShouldCountInner > From < Arc < T > > for SizableArc < T , SC > { fn from (value : Arc < T >) -> Self { SizableArc (value , PhantomData) } }
};
}
