// Generated macro for impl_285 (impl)
macro_rules! Depcrate_raw_stringimpl_285 {
() => {
// Module: crate::raw_string
// Provides: {"impl_285"}
// Dependencies: {}
impl From < & str > for RawString { # [inline] fn from (s : & str) -> Self { if s . is_empty () { Self (RawStringInner :: Empty) } else { String :: from (s) . into () } } }
};
}
