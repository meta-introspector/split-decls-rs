// Generated macro for impl_459 (impl)
macro_rules! Depcrate_wrappersimpl_459 {
() => {
// Module: crate::wrappers
// Provides: {"impl_459"}
// Dependencies: {}
impl < T : Unaligned + Ord > Ord for Unalign < T > { # [inline (always)] fn cmp (& self , other : & Unalign < T >) -> Ordering { Ord :: cmp (self . deref () , other . deref ()) } }
};
}
