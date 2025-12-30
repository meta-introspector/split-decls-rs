// Generated macro for impl_221 (impl)
macro_rules! Depcrate_span_encodingimpl_221 {
() => {
// Module: crate::span_encoding
// Provides: {"impl_221"}
// Dependencies: {}
impl PartiallyInterned { # [inline] fn data (self) -> SpanData { SpanData { ctxt : SyntaxContext :: from_u16 (self . ctxt) , .. with_span_interner (| interner | interner . spans [self . index as usize]) } } # [inline] fn span (index : u32 , ctxt : u16) -> Span { let (lo_or_index , len_with_tag_or_marker , ctxt_or_parent_or_marker) = (index , BASE_LEN_INTERNED_MARKER , ctxt) ; Span { lo_or_index , len_with_tag_or_marker , ctxt_or_parent_or_marker } } # [inline] fn from_span (span : Span) -> PartiallyInterned { PartiallyInterned { index : span . lo_or_index , ctxt : span . ctxt_or_parent_or_marker } } }
};
}
