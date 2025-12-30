// Generated macro for impl_561 (impl)
macro_rules! Depcrate_streamimpl_561 {
() => {
// Module: crate::stream
// Provides: {"impl_561"}
// Dependencies: {}
impl < 'b > Compare < & 'b [u8] > for & [u8] { # [inline] fn compare (& self , t : & 'b [u8]) -> CompareResult { if t . iter () . zip (* self) . any (| (a , b) | a != b) { CompareResult :: Error } else if self . len () < t . slice_len () { CompareResult :: Incomplete } else { CompareResult :: Ok (t . slice_len ()) } } }
};
}
