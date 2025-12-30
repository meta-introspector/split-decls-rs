// Generated macro for bracketed_glob (function)
macro_rules! Depcrate_parser_wordbracketed_glob {
() => {
// Module: crate::parser::word
// Provides: {"bracketed_glob"}
// Dependencies: {}
fn bracketed_glob (span : Span) -> ParseResult < String > { map (delimited (char ('[') , pair (opt (one_of ("!^")) , alt ((map (char (']') , | c | vinto ! [c]) , many1 (alt ((recognize_string (delimited (tag ("[:") , alpha1 , tag (":]"))) , lit_string ("]") , map (alt ((char ('[') , one_of (EXTGLOB_PREFIX))) , | c | c . into ()) ,))) ,)) ,) , char (']') ,) , | (neg_prefix , mut sgmts) | { if let Some (negation) = neg_prefix { sgmts . insert (0 , negation . into ()) ; } format ! ("[{}]" , sgmts . concat ()) } ,) (span) }
};
}
