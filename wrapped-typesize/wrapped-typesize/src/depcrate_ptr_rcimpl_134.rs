// Generated macro for impl_134 (impl)
macro_rules! Depcrate_ptr_rcimpl_134 {
() => {
// Module: crate::ptr::rc
// Provides: {"impl_134"}
// Dependencies: {}
impl < T , SC : ShouldCountInner > From < Rc < T > > for SizableRc < T , SC > { fn from (value : Rc < T >) -> Self { SizableRc (value , PhantomData) } }
};
}
