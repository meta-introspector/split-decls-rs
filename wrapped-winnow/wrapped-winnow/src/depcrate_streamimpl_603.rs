// Generated macro for impl_603 (impl)
macro_rules! Depcrate_streamimpl_603 {
() => {
// Module: crate::stream
// Provides: {"impl_603"}
// Dependencies: {}
impl < R : FromStr > ParseSlice < R > for & [u8] { # [inline (always)] fn parse_slice (& self) -> Option < R > { from_utf8 (self) . ok () . and_then (| s | s . parse () . ok ()) } }
};
}
