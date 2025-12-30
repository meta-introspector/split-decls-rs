// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < S > SpanMapper < SpanData < S > > for SpanMap < S > where SpanData < S > : Copy , { fn span_for (& self , range : TextRange) -> SpanData < S > { self . span_at (range . start ()) } }
};
}
