// Generated macro for impl_287 (impl)
macro_rules! Depcrate_raw_stringimpl_287 {
() => {
// Module: crate::raw_string
// Provides: {"impl_287"}
// Dependencies: {}
impl From < & String > for RawString { # [inline] fn from (s : & String) -> Self { if s . is_empty () { Self (RawStringInner :: Empty) } else { String :: from (s) . into () } } }
};
}
