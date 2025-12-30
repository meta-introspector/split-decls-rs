// Generated macro for rewrite_bounds_on_where_clause (function)
macro_rules! Depcrate_itemsrewrite_bounds_on_where_clause {
() => {
// Module: crate::items
// Provides: {"rewrite_bounds_on_where_clause"}
// Dependencies: {}
# [doc = " Rewrite bounds on a where clause."] fn rewrite_bounds_on_where_clause (context : & RewriteContext < '_ > , predicates : & [ast :: WherePredicate] , shape : Shape , terminator : & str , span_end : Option < BytePos > , where_clause_option : WhereClauseOption , force_single_line : bool ,) -> RewriteResult { let span_start = predicates [0] . span () . lo () ; let len = predicates . len () ; let end_of_preds = predicates [len - 1] . span () . hi () ; let span_end = span_end . unwrap_or (end_of_preds) ; let items = itemize_list (context . snippet_provider , predicates . iter () , terminator , "," , | pred | pred . span () . lo () , | pred | pred . span () . hi () , | pred | pred . rewrite_result (context , shape) , span_start , span_end , false ,) ; let comma_tactic = if where_clause_option . suppress_comma || force_single_line { SeparatorTactic :: Never } else { context . config . trailing_comma () } ; let shape_tactic = if force_single_line { DefinitiveListTactic :: Horizontal } else { DefinitiveListTactic :: Vertical } ; let preserve_newline = context . config . style_edition () <= StyleEdition :: Edition2021 ; let fmt = ListFormatting :: new (shape , context . config) . tactic (shape_tactic) . trailing_separator (comma_tactic) . preserve_newline (preserve_newline) ; write_list (& items . collect :: < Vec < _ > > () , & fmt) }
};
}
