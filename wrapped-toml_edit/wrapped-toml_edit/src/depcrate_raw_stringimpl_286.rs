// Generated macro for impl_286 (impl)
macro_rules! Depcrate_raw_stringimpl_286 {
() => {
// Module: crate::raw_string
// Provides: {"impl_286"}
// Dependencies: {}
impl From < String > for RawString { # [inline] fn from (s : String) -> Self { if s . is_empty () { Self (RawStringInner :: Empty) } else { Self (RawStringInner :: Explicit (s)) } } }
};
}
