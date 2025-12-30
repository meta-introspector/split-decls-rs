// Generated macro for impl_566 (impl)
macro_rules! Depcrate_streamimpl_566 {
() => {
// Module: crate::stream
// Provides: {"impl_566"}
// Dependencies: {}
impl < 'b , const LEN : usize > Compare < AsciiCaseless < & 'b [u8 ; LEN] > > for & [u8] { # [inline (always)] fn compare (& self , t : AsciiCaseless < & 'b [u8 ; LEN] >) -> CompareResult { self . compare (AsciiCaseless (& t . 0 [..])) } }
};
}
