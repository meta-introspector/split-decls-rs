// Generated macro for impl_569 (impl)
macro_rules! Depcrate_streamimpl_569 {
() => {
// Module: crate::stream
// Provides: {"impl_569"}
// Dependencies: {}
impl Compare < u8 > for & [u8] { # [inline] fn compare (& self , t : u8) -> CompareResult { match self . first () . copied () { Some (c) if t == c => CompareResult :: Ok (t . slice_len ()) , Some (_) => CompareResult :: Error , None => CompareResult :: Incomplete , } } }
};
}
