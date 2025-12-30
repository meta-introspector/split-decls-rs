// Generated macro for impl_571 (impl)
macro_rules! Depcrate_streamimpl_571 {
() => {
// Module: crate::stream
// Provides: {"impl_571"}
// Dependencies: {}
impl Compare < char > for & [u8] { # [inline (always)] fn compare (& self , t : char) -> CompareResult { self . compare (t . encode_utf8 (& mut [0 ; 4]) . as_bytes ()) } }
};
}
