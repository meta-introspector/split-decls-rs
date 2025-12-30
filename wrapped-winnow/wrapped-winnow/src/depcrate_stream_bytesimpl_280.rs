// Generated macro for impl_280 (impl)
macro_rules! Depcrate_stream_bytesimpl_280 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_280"}
// Dependencies: {}
impl < 'a , T > Compare < T > for & 'a Bytes where & 'a [u8] : Compare < T > , { # [inline (always)] fn compare (& self , t : T) -> CompareResult { let bytes = (* self) . as_bytes () ; bytes . compare (t) } }
};
}
