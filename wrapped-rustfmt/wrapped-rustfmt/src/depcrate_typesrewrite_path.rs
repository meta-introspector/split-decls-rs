// Generated macro for rewrite_path (function)
macro_rules! Depcrate_typesrewrite_path {
() => {
// Module: crate::types
// Provides: {"rewrite_path"}
// Dependencies: {}
pub (crate) fn rewrite_path (context : & RewriteContext < '_ > , path_context : PathContext , qself : & Option < ptr :: P < ast :: QSelf > > , path : & ast :: Path , shape : Shape ,) -> RewriteResult { let skip_count = qself . as_ref () . map_or (0 , | x | x . position) ; let mut result = String :: with_capacity (32) ; if path . is_global () && qself . is_none () && path_context != PathContext :: Import { result . push_str ("::") ; } let mut span_lo = path . span . lo () ; if let Some (qself) = qself { result . push ('<') ; let fmt_ty = qself . ty . rewrite_result (context , shape) ? ; result . push_str (& fmt_ty) ; if skip_count > 0 { result . push_str (" as ") ; if path . is_global () && path_context != PathContext :: Import { result . push_str ("::") ; } let shape = shape . sub_width (3 , path . span) ? ; result = rewrite_path_segments (PathContext :: Type , result , path . segments . iter () . take (skip_count) , span_lo , path . span . hi () , context , shape ,) ? ; } result . push_str (">::") ; span_lo = qself . ty . span . hi () + BytePos (1) ; } rewrite_path_segments (path_context , result , path . segments . iter () . skip (skip_count) , span_lo , path . span . hi () , context , shape ,) }
};
}
