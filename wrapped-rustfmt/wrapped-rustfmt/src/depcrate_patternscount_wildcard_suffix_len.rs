// Generated macro for count_wildcard_suffix_len (function)
macro_rules! Depcrate_patternscount_wildcard_suffix_len {
() => {
// Module: crate::patterns
// Provides: {"count_wildcard_suffix_len"}
// Dependencies: {}
fn count_wildcard_suffix_len (context : & RewriteContext < '_ > , patterns : & [TuplePatField < '_ >] , span : Span , shape : Shape ,) -> usize { let mut suffix_len = 0 ; let items : Vec < _ > = itemize_list (context . snippet_provider , patterns . iter () , ")" , "," , | item | item . span () . lo () , | item | item . span () . hi () , | item | item . rewrite_result (context , shape) , context . snippet_provider . span_after (span , "(") , span . hi () - BytePos (1) , false ,) . collect () ; for item in items . iter () . rev () . take_while (| i | matches ! (i . item , Ok (ref internal_string) if internal_string == "_")) { suffix_len += 1 ; if item . has_comment () { break ; } } suffix_len }
};
}
