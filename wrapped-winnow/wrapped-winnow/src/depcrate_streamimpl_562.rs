// Generated macro for impl_562 (impl)
macro_rules! Depcrate_streamimpl_562 {
() => {
// Module: crate::stream
// Provides: {"impl_562"}
// Dependencies: {}
impl < 'b > Compare < AsciiCaseless < & 'b [u8] > > for & [u8] { # [inline] fn compare (& self , t : AsciiCaseless < & 'b [u8] >) -> CompareResult { if t . 0 . iter () . zip (* self) . any (| (a , b) | ! a . eq_ignore_ascii_case (b)) { CompareResult :: Error } else if self . len () < t . slice_len () { CompareResult :: Incomplete } else { CompareResult :: Ok (t . slice_len ()) } } }
};
}
