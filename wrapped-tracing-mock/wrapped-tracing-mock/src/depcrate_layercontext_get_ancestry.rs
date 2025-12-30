// Generated macro for context_get_ancestry (function)
macro_rules! Depcrate_layercontext_get_ancestry {
() => {
// Module: crate::layer
// Provides: {"context_get_ancestry"}
// Dependencies: {}
fn context_get_ancestry < C > (item : impl HasAncestry , ctx : & Context < '_ , C >) -> ActualAncestry where C : Subscriber + for < 'a > LookupSpan < 'a > , { get_ancestry (item , | | ctx . lookup_current () . map (| s | s . id ()) , | span_id | ctx . span (span_id) . map (| span | (& span) . into ()) ,) }
};
}
