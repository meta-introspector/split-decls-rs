// Generated macro for rewrite_where_clause_rfc_style (function)
macro_rules! Depcrate_itemsrewrite_where_clause_rfc_style {
() => {
// Module: crate::items
// Provides: {"rewrite_where_clause_rfc_style"}
// Dependencies: {}
fn rewrite_where_clause_rfc_style (context : & RewriteContext < '_ > , predicates : & [ast :: WherePredicate] , where_span : Span , shape : Shape , terminator : & str , span_end : Option < BytePos > , span_end_before_where : BytePos , where_clause_option : WhereClauseOption ,) -> RewriteResult { let (where_keyword , allow_single_line) = rewrite_where_keyword (context , predicates , where_span , shape , span_end_before_where , where_clause_option ,) ? ; let clause_shape = shape . block () . with_max_width (context . config) . block_left (context . config . tab_spaces () , where_span) ? . sub_width (1 , where_span) ? ; let force_single_line = context . config . where_single_line () && predicates . len () == 1 && ! where_clause_option . veto_single_line ; let preds_str = rewrite_bounds_on_where_clause (context , predicates , clause_shape , terminator , span_end , where_clause_option , force_single_line ,) ? ; let clause_sep = if allow_single_line && ! preds_str . contains ('\n') && 6 + preds_str . len () <= shape . width || force_single_line { Cow :: from (" ") } else { clause_shape . indent . to_string_with_newline (context . config) } ; Ok (format ! ("{where_keyword}{clause_sep}{preds_str}")) }
};
}
