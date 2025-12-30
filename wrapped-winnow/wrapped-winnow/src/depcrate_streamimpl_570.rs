// Generated macro for impl_570 (impl)
macro_rules! Depcrate_streamimpl_570 {
() => {
// Module: crate::stream
// Provides: {"impl_570"}
// Dependencies: {}
impl Compare < AsciiCaseless < u8 > > for & [u8] { # [inline] fn compare (& self , t : AsciiCaseless < u8 >) -> CompareResult { match self . first () { Some (c) if t . 0 . eq_ignore_ascii_case (c) => CompareResult :: Ok (t . slice_len ()) , Some (_) => CompareResult :: Error , None => CompareResult :: Incomplete , } } }
};
}
