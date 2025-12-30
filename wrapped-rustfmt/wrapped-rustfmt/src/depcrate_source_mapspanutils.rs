// Generated macro for SpanUtils (trait)
macro_rules! Depcrate_source_mapSpanUtils {
() => {
// Module: crate::source_map
// Provides: {"SpanUtils"}
// Dependencies: {}
pub (crate) trait SpanUtils { fn span_after (& self , original : Span , needle : & str) -> BytePos ; fn span_after_last (& self , original : Span , needle : & str) -> BytePos ; fn span_before (& self , original : Span , needle : & str) -> BytePos ; fn span_before_last (& self , original : Span , needle : & str) -> BytePos ; fn opt_span_after (& self , original : Span , needle : & str) -> Option < BytePos > ; fn opt_span_before (& self , original : Span , needle : & str) -> Option < BytePos > ; }
};
}
