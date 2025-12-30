// Generated macro for impl_460 (impl)
macro_rules! Depcrate_wrappersimpl_460 {
() => {
// Module: crate::wrappers
// Provides: {"impl_460"}
// Dependencies: {}
impl < T : Unaligned + PartialEq > PartialEq < Unalign < T > > for Unalign < T > { # [inline (always)] fn eq (& self , other : & Unalign < T >) -> bool { PartialEq :: eq (self . deref () , other . deref ()) } }
};
}
