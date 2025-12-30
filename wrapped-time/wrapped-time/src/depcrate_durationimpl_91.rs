// Generated macro for impl_91 (impl)
macro_rules! Depcrate_durationimpl_91 {
() => {
// Module: crate::duration
// Provides: {"impl_91"}
// Dependencies: {}
impl PartialOrd < Duration > for StdDuration { # [inline] fn partial_cmp (& self , rhs : & Duration) -> Option < Ordering > { rhs . partial_cmp (self) . map (Ordering :: reverse) } }
};
}
