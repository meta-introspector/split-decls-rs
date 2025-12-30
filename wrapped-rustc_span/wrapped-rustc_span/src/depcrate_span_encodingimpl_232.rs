// Generated macro for impl_232 (impl)
macro_rules! Depcrate_span_encodingimpl_232 {
() => {
// Module: crate::span_encoding
// Provides: {"impl_232"}
// Dependencies: {}
impl SpanInterner { fn intern (& mut self , span_data : & SpanData) -> u32 { let (index , _) = self . spans . insert_full (* span_data) ; index as u32 } }
};
}
