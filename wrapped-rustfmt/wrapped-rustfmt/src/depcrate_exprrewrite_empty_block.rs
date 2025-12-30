// Generated macro for rewrite_empty_block (function)
macro_rules! Depcrate_exprrewrite_empty_block {
() => {
// Module: crate::expr
// Provides: {"rewrite_empty_block"}
// Dependencies: {}
fn rewrite_empty_block (context : & RewriteContext < '_ > , block : & ast :: Block , attrs : Option < & [ast :: Attribute] > , label : Option < ast :: Label > , prefix : & str , shape : Shape ,) -> Option < String > { if block_has_statements (block) { return None ; } let label_str = rewrite_label (context , label) ; if attrs . map_or (false , | a | ! inner_attributes (a) . is_empty ()) { return None ; } if ! block_contains_comment (context , block) && shape . width >= 2 { return Some (format ! ("{prefix}{label_str}{{}}")) ; } let user_str = context . snippet (block . span) ; let user_str = user_str . trim () ; if user_str . starts_with ('{') && user_str . ends_with ('}') { let comment_str = user_str [1 .. user_str . len () - 1] . trim () ; if block . stmts . is_empty () && ! comment_str . contains ('\n') && ! comment_str . starts_with ("//") && comment_str . len () + 4 <= shape . width { return Some (format ! ("{prefix}{label_str}{{ {comment_str} }}")) ; } } None }
};
}
