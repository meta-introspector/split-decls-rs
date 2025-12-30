// Generated macro for rewrite_tuple_pat (function)
macro_rules! Depcrate_patternsrewrite_tuple_pat {
() => {
// Module: crate::patterns
// Provides: {"rewrite_tuple_pat"}
// Dependencies: {}
fn rewrite_tuple_pat (pats : & [ptr :: P < ast :: Pat >] , path_str : Option < String > , span : Span , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { if pats . is_empty () { return Ok (format ! ("{}()" , path_str . unwrap_or_default ())) ; } let mut pat_vec : Vec < _ > = pats . iter () . map (TuplePatField :: Pat) . collect () ; let wildcard_suffix_len = count_wildcard_suffix_len (context , & pat_vec , span , shape) ; let (pat_vec , span) = if context . config . condense_wildcard_suffixes () && wildcard_suffix_len >= 2 { let new_item_count = 1 + pat_vec . len () - wildcard_suffix_len ; let sp = pat_vec [new_item_count - 1] . span () ; let snippet = context . snippet (sp) ; let lo = sp . lo () + BytePos (snippet . find_uncommented ("_") . unwrap () as u32) ; pat_vec [new_item_count - 1] = TuplePatField :: Dotdot (mk_sp_lo_plus_one (lo)) ; (& pat_vec [.. new_item_count] , mk_sp (span . lo () , lo + BytePos (1)) ,) } else { (& pat_vec [..] , span) } ; let is_last_pat_dotdot = pat_vec . last () . map_or (false , | p | p . is_dotdot ()) ; let add_comma = path_str . is_none () && pat_vec . len () == 1 && ! is_last_pat_dotdot ; let path_str = path_str . unwrap_or_default () ; overflow :: rewrite_with_parens (context , & path_str , pat_vec . iter () , shape , span , context . config . max_width () , if add_comma { Some (SeparatorTactic :: Always) } else { None } ,) }
};
}
