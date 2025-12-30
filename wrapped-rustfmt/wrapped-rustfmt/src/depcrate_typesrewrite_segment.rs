// Generated macro for rewrite_segment (function)
macro_rules! Depcrate_typesrewrite_segment {
() => {
// Module: crate::types
// Provides: {"rewrite_segment"}
// Dependencies: {}
fn rewrite_segment (path_context : PathContext , segment : & ast :: PathSegment , span_lo : & mut BytePos , span_hi : BytePos , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { let mut result = String :: with_capacity (128) ; result . push_str (rewrite_ident (context , segment . ident)) ; let ident_len = result . len () ; let span = mk_sp (* span_lo , span_hi) ; let shape = if context . use_block_indent () { shape . offset_left (ident_len , span) ? } else { shape . shrink_left (ident_len , span) ? } ; if let Some (ref args) = segment . args { let generics_str = rewrite_generic_args (args , context , shape , mk_sp (* span_lo , span_hi)) ? ; match * * args { ast :: GenericArgs :: AngleBracketed (ref data) if ! data . args . is_empty () => { let separator_snippet = context . snippet (mk_sp (segment . ident . span . hi () , data . span . lo ())) . trim () ; let force_separator = context . inside_macro () && separator_snippet . starts_with ("::") ; let separator = if path_context == PathContext :: Expr || force_separator { "::" } else { "" } ; result . push_str (separator) ; * span_lo = context . snippet_provider . span_after (mk_sp (* span_lo , span_hi) , "<") ; } _ => () , } result . push_str (& generics_str) } Ok (result) }
};
}
