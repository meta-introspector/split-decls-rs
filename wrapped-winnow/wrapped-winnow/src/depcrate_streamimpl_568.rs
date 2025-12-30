// Generated macro for impl_568 (impl)
macro_rules! Depcrate_streamimpl_568 {
() => {
// Module: crate::stream
// Provides: {"impl_568"}
// Dependencies: {}
impl < 'b > Compare < AsciiCaseless < & 'b str > > for & [u8] { # [inline (always)] fn compare (& self , t : AsciiCaseless < & 'b str >) -> CompareResult { self . compare (AsciiCaseless (t . 0 . as_bytes ())) } }
};
}
