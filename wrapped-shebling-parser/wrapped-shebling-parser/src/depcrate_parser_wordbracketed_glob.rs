// Generated macro for bracketed_glob (function)
macro_rules! Depcrate_parser_wordbracketed_glob {
() => {
// Module: crate::parser::word
// Provides: {"bracketed_glob"}
// Dependencies: {}
fn bracketed_glob (span : ParseSpan) -> ParseResult < String > { map (delimited (char ('[') , pair (opt (one_of ("!^")) , alt ((map (char (']') , | c | vec ! [c . into ()]) , many1 (alt ((recognize_string (delimited (tag ("[:") , alpha1 , tag (":]"))) , lit_string ("]") , recognize_string (one_of (& * format ! ("[{}" , expansion :: EXTGLOB_PREFIX))) ,))) ,)) ,) , char (']') ,) , | (neg_prefix , mut sgmts) | { if let Some (negation) = neg_prefix { sgmts . insert (0 , negation . into ()) ; } format ! ("[{}]" , sgmts . concat ()) } ,) (span) }
};
}
