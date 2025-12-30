// Generated macro for impl_564 (impl)
macro_rules! Depcrate_streamimpl_564 {
() => {
// Module: crate::stream
// Provides: {"impl_564"}
// Dependencies: {}
impl < const LEN : usize > Compare < AsciiCaseless < [u8 ; LEN] > > for & [u8] { # [inline (always)] fn compare (& self , t : AsciiCaseless < [u8 ; LEN] >) -> CompareResult { self . compare (AsciiCaseless (& t . 0 [..])) } }
};
}
