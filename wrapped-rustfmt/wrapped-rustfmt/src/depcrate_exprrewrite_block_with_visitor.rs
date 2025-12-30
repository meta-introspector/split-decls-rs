// Generated macro for rewrite_block_with_visitor (function)
macro_rules! Depcrate_exprrewrite_block_with_visitor {
() => {
// Module: crate::expr
// Provides: {"rewrite_block_with_visitor"}
// Dependencies: {}
pub (crate) fn rewrite_block_with_visitor (context : & RewriteContext < '_ > , prefix : & str , block : & ast :: Block , attrs : Option < & [ast :: Attribute] > , label : Option < ast :: Label > , shape : Shape , has_braces : bool ,) -> RewriteResult { if let Some (rw_str) = rewrite_empty_block (context , block , attrs , label , prefix , shape) { return Ok (rw_str) ; } let mut visitor = FmtVisitor :: from_context (context) ; visitor . block_indent = shape . indent ; visitor . is_if_else_block = context . is_if_else_block () ; match (block . rules , label) { (ast :: BlockCheckMode :: Unsafe (..) , _) | (ast :: BlockCheckMode :: Default , Some (_)) => { let snippet = context . snippet (block . span) ; let open_pos = snippet . find_uncommented ("{") . unknown_error () ? ; visitor . last_pos = block . span . lo () + BytePos (open_pos as u32) } (ast :: BlockCheckMode :: Default , None) => visitor . last_pos = block . span . lo () , } let inner_attrs = attrs . map (inner_attributes) ; let label_str = rewrite_label (context , label) ; visitor . visit_block (block , inner_attrs . as_deref () , has_braces) ; let visitor_context = visitor . get_context () ; context . skipped_range . borrow_mut () . append (& mut visitor_context . skipped_range . borrow_mut ()) ; Ok (format ! ("{}{}{}" , prefix , label_str , visitor . buffer)) }
};
}
