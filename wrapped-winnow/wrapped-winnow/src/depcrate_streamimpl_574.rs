// Generated macro for impl_574 (impl)
macro_rules! Depcrate_streamimpl_574 {
() => {
// Module: crate::stream
// Provides: {"impl_574"}
// Dependencies: {}
impl < 'b > Compare < AsciiCaseless < & 'b str > > for & str { # [inline (always)] fn compare (& self , t : AsciiCaseless < & 'b str >) -> CompareResult { self . as_bytes () . compare (t . as_bytes ()) } }
};
}
