// Generated macro for impl_572 (impl)
macro_rules! Depcrate_streamimpl_572 {
() => {
// Module: crate::stream
// Provides: {"impl_572"}
// Dependencies: {}
impl Compare < AsciiCaseless < char > > for & [u8] { # [inline (always)] fn compare (& self , t : AsciiCaseless < char >) -> CompareResult { self . compare (AsciiCaseless (t . 0 . encode_utf8 (& mut [0 ; 4]) . as_bytes ())) } }
};
}
