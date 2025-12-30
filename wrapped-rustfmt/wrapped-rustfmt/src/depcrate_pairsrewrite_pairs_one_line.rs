// Generated macro for rewrite_pairs_one_line (function)
macro_rules! Depcrate_pairsrewrite_pairs_one_line {
() => {
// Module: crate::pairs
// Provides: {"rewrite_pairs_one_line"}
// Dependencies: {}
fn rewrite_pairs_one_line < T : Rewrite > (list : & PairList < '_ , '_ , T > , shape : Shape , context : & RewriteContext < '_ > ,) -> Option < String > { assert ! (list . list . len () >= 2 , "Not a pair?") ; let mut result = String :: new () ; let base_shape = shape . block () ; for ((_ , rewrite) , s) in list . list . iter () . zip (list . separators . iter ()) { if let Ok (rewrite) = rewrite { if ! is_single_line (rewrite) || result . len () > shape . width { return None ; } result . push_str (rewrite) ; result . push (' ') ; result . push_str (s) ; result . push (' ') ; } else { return None ; } } let prefix_len = result . len () ; let last = list . list . last () ? . 0 ; let cur_shape = base_shape . offset_left_opt (last_line_width (& result)) ? ; let last_rewrite = last . rewrite (context , cur_shape) ? ; result . push_str (& last_rewrite) ; if first_line_width (& result) > shape . width { return None ; } if ! (is_single_line (& result) || last_rewrite . starts_with ('{')) && (last_rewrite . starts_with ('(') || prefix_len > context . config . tab_spaces ()) { return None ; } wrap_str (result , context . config . max_width () , shape) }
};
}
