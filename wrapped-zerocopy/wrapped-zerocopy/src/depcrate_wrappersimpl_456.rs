// Generated macro for impl_456 (impl)
macro_rules! Depcrate_wrappersimpl_456 {
() => {
// Module: crate::wrappers
// Provides: {"impl_456"}
// Dependencies: {}
impl < T : Unaligned > Deref for Unalign < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { Ptr :: from_ref (self) . transmute () . bikeshed_recall_aligned () . as_ref () } }
};
}
