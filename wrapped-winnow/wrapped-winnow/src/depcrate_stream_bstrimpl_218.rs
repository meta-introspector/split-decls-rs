// Generated macro for impl_218 (impl)
macro_rules! Depcrate_stream_bstrimpl_218 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_218"}
// Dependencies: {}
impl < 'i , S > FindSlice < S > for & 'i BStr where & 'i [u8] : FindSlice < S > , { # [inline (always)] fn find_slice (& self , substr : S) -> Option < core :: ops :: Range < usize > > { let bytes = (* self) . as_bytes () ; let offset = bytes . find_slice (substr) ; offset } }
};
}
