// Generated macro for impl_458 (impl)
macro_rules! Depcrate_wrappersimpl_458 {
() => {
// Module: crate::wrappers
// Provides: {"impl_458"}
// Dependencies: {}
impl < T : Unaligned + PartialOrd > PartialOrd < Unalign < T > > for Unalign < T > { # [inline (always)] fn partial_cmp (& self , other : & Unalign < T >) -> Option < Ordering > { PartialOrd :: partial_cmp (self . deref () , other . deref ()) } }
};
}
