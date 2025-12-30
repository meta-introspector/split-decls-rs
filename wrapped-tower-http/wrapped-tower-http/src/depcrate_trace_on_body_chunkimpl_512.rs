// Generated macro for impl_512 (impl)
macro_rules! Depcrate_trace_on_body_chunkimpl_512 {
() => {
// Module: crate::trace::on_body_chunk
// Provides: {"impl_512"}
// Dependencies: {}
impl < B , F > OnBodyChunk < B > for F where F : FnMut (& B , Duration , & Span) , { fn on_body_chunk (& mut self , chunk : & B , latency : Duration , span : & Span) { self (chunk , latency , span) } }
};
}
