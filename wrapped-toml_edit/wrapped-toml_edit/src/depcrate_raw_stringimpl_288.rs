// Generated macro for impl_288 (impl)
macro_rules! Depcrate_raw_stringimpl_288 {
() => {
// Module: crate::raw_string
// Provides: {"impl_288"}
// Dependencies: {}
impl From < Box < str > > for RawString { # [inline] fn from (s : Box < str >) -> Self { if s . is_empty () { Self (RawStringInner :: Empty) } else { String :: from (s) . into () } } }
};
}
