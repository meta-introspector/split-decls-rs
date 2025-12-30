// Generated macro for rewrite_bounded_lifetime (function)
macro_rules! Depcrate_typesrewrite_bounded_lifetime {
() => {
// Module: crate::types
// Provides: {"rewrite_bounded_lifetime"}
// Dependencies: {}
fn rewrite_bounded_lifetime (lt : & ast :: Lifetime , bounds : & [ast :: GenericBound] , span : Span , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { let result = lt . rewrite_result (context , shape) ? ; if bounds . is_empty () { Ok (result) } else { let colon = type_bound_colon (context) ; let overhead = last_line_width (& result) + colon . len () ; let shape = shape . sub_width (overhead , span) ? ; let result = format ! ("{}{}{}" , result , colon , join_bounds (context , shape , bounds , true) ?) ; Ok (result) } }
};
}
