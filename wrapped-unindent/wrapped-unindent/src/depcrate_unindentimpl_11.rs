// Generated macro for impl_11 (impl)
macro_rules! Depcrate_unindentimpl_11 {
() => {
// Module: crate::unindent
// Provides: {"impl_11"}
// Dependencies: {}
impl < T : ? Sized + Unindent > Unindent for & T { type Output = T :: Output ; fn unindent (& self) -> Self :: Output { (* * self) . unindent () } }
};
}
