// Generated macro for impl_281 (impl)
macro_rules! Depcrate_stream_bytesimpl_281 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_281"}
// Dependencies: {}
impl < 'i , S > FindSlice < S > for & 'i Bytes where & 'i [u8] : FindSlice < S > , { # [inline (always)] fn find_slice (& self , substr : S) -> Option < core :: ops :: Range < usize > > { let bytes = (* self) . as_bytes () ; let offset = bytes . find_slice (substr) ; offset } }
};
}
