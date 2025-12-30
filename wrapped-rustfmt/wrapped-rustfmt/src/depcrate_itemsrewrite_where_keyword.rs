// Generated macro for rewrite_where_keyword (function)
macro_rules! Depcrate_itemsrewrite_where_keyword {
() => {
// Module: crate::items
// Provides: {"rewrite_where_keyword"}
// Dependencies: {}
# [doc = " Rewrite `where` and comment around it."] fn rewrite_where_keyword (context : & RewriteContext < '_ > , predicates : & [ast :: WherePredicate] , where_span : Span , shape : Shape , span_end_before_where : BytePos , where_clause_option : WhereClauseOption ,) -> Result < (String , bool) , RewriteError > { let block_shape = shape . block () . with_max_width (context . config) ; let clause_shape = block_shape . block_left (context . config . tab_spaces () , where_span) ? . sub_width (1 , where_span) ? ; let comment_separator = | comment : & str , shape : Shape | { if comment . is_empty () { Cow :: from ("") } else { shape . indent . to_string_with_newline (context . config) } } ; let (span_before , span_after) = missing_span_before_after_where (span_end_before_where , predicates , where_span) ; let (comment_before , comment_after) = rewrite_comments_before_after_where (context , span_before , span_after , shape) ? ; let starting_newline = match where_clause_option . snuggle { WhereClauseSpace :: Space if comment_before . is_empty () => Cow :: from (" ") , WhereClauseSpace :: None => Cow :: from ("") , _ => block_shape . indent . to_string_with_newline (context . config) , } ; let newline_before_where = comment_separator (& comment_before , shape) ; let newline_after_where = comment_separator (& comment_after , clause_shape) ; let result = format ! ("{starting_newline}{comment_before}{newline_before_where}where\
{newline_after_where}{comment_after}") ; let allow_single_line = where_clause_option . allow_single_line && comment_before . is_empty () && comment_after . is_empty () ; Ok ((result , allow_single_line)) }
};
}
