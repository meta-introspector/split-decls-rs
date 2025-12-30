// Generated macro for impl_563 (impl)
macro_rules! Depcrate_streamimpl_563 {
() => {
// Module: crate::stream
// Provides: {"impl_563"}
// Dependencies: {}
impl < const LEN : usize > Compare < [u8 ; LEN] > for & [u8] { # [inline (always)] fn compare (& self , t : [u8 ; LEN]) -> CompareResult { self . compare (& t [..]) } }
};
}
