// Generated macro for impl_604 (impl)
macro_rules! Depcrate_streamimpl_604 {
() => {
// Module: crate::stream
// Provides: {"impl_604"}
// Dependencies: {}
impl < R : FromStr > ParseSlice < R > for & str { # [inline (always)] fn parse_slice (& self) -> Option < R > { self . parse () . ok () } }
};
}
