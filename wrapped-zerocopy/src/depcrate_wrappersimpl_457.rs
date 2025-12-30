// Generated macro for impl_457 (impl)
macro_rules! Depcrate_wrappersimpl_457 {
() => {
// Module: crate::wrappers
// Provides: {"impl_457"}
// Dependencies: {}
impl < T : Unaligned > DerefMut for Unalign < T > { # [inline (always)] fn deref_mut (& mut self) -> & mut T { Ptr :: from_mut (self) . transmute :: < _ , _ , (_ , (_ , _)) > () . bikeshed_recall_aligned () . as_mut () } }
};
}
