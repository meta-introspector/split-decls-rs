// Generated macro for enum_variant_span (function)
macro_rules! Depcrate_itemsenum_variant_span {
() => {
// Module: crate::items
// Provides: {"enum_variant_span"}
// Dependencies: {}
fn enum_variant_span (variant : & ast :: Variant , context : & RewriteContext < '_ >) -> Span { use ast :: VariantData :: * ; if let Some (ref anon_const) = variant . disr_expr { let span_before_consts = variant . span . until (anon_const . value . span) ; let hi = match & variant . data { Struct { .. } => context . snippet_provider . span_after_last (span_before_consts , "}") , Tuple (..) => context . snippet_provider . span_after_last (span_before_consts , ")") , Unit (..) => variant . ident . span . hi () , } ; mk_sp (span_before_consts . lo () , hi) } else { variant . span } }
};
}
