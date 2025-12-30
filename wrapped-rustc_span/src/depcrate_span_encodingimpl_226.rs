// Generated macro for impl_226 (impl)
macro_rules! Depcrate_span_encodingimpl_226 {
() => {
// Module: crate::span_encoding
// Provides: {"impl_226"}
// Dependencies: {}
impl Interned { # [inline] fn data (self) -> SpanData { with_span_interner (| interner | interner . spans [self . index as usize]) } # [inline] fn span (index : u32) -> Span { let (lo_or_index , len_with_tag_or_marker , ctxt_or_parent_or_marker) = (index , BASE_LEN_INTERNED_MARKER , CTXT_INTERNED_MARKER) ; Span { lo_or_index , len_with_tag_or_marker , ctxt_or_parent_or_marker } } # [inline] fn from_span (span : Span) -> Interned { Interned { index : span . lo_or_index } } }
};
}
