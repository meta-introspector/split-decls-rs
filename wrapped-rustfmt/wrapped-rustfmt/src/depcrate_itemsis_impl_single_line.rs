// Generated macro for is_impl_single_line (function)
macro_rules! Depcrate_itemsis_impl_single_line {
() => {
// Module: crate::items
// Provides: {"is_impl_single_line"}
// Dependencies: {}
fn is_impl_single_line (context : & RewriteContext < '_ > , items : & [ptr :: P < ast :: AssocItem >] , result : & str , where_clause_str : & str , item : & ast :: Item ,) -> Result < bool , RewriteError > { let snippet = context . snippet (item . span) ; let open_pos = snippet . find_uncommented ("{") . unknown_error () ? + 1 ; Ok (context . config . empty_item_single_line () && items . is_empty () && ! result . contains ('\n') && result . len () + where_clause_str . len () <= context . config . max_width () && ! contains_comment (& snippet [open_pos ..])) }
};
}
