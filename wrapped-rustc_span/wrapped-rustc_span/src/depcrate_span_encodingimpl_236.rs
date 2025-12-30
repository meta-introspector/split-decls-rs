// Generated macro for impl_236 (impl)
macro_rules! Depcrate_span_encodingimpl_236 {
() => {
// Module: crate::span_encoding
// Provides: {"impl_236"}
// Dependencies: {}
impl SpanInterner { fn intern (& mut self , span_data : & SpanData) -> u32 { let (index , _) = self . spans . insert_full (* span_data) ; index as u32 } }
};
}
