// Generated macro for impl_565 (impl)
macro_rules! Depcrate_streamimpl_565 {
() => {
// Module: crate::stream
// Provides: {"impl_565"}
// Dependencies: {}
impl < 'b , const LEN : usize > Compare < & 'b [u8 ; LEN] > for & [u8] { # [inline (always)] fn compare (& self , t : & 'b [u8 ; LEN]) -> CompareResult { self . compare (& t [..]) } }
};
}
