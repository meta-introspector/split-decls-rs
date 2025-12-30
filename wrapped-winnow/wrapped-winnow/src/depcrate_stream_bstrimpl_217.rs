// Generated macro for impl_217 (impl)
macro_rules! Depcrate_stream_bstrimpl_217 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_217"}
// Dependencies: {}
impl < 'a , T > Compare < T > for & 'a BStr where & 'a [u8] : Compare < T > , { # [inline (always)] fn compare (& self , t : T) -> CompareResult { let bytes = (* self) . as_bytes () ; bytes . compare (t) } }
};
}
