// Generated macro for _parse (function)
macro_rules! Depcrate_parse {
() => {
// Module: crate
// Provides: {"_parse"}
// Dependencies: {}
# [cfg (feature = "parsing")] fn _parse < T > (tokens : proc_macro2 :: TokenStream) -> Result < T , ParseError > where T : Synom , { let buf = SynomBuffer :: new (tokens) ; let result = T :: parse (buf . begin ()) ; let err = match result { Ok ((rest , t)) => { if rest . eof () { return Ok (t) ; } else if rest == buf . begin () { ParseError :: new ("failed to parse anything") } else { ParseError :: new ("failed to parse all tokens") } } Err (err) => err , } ; match T :: description () { Some (s) => Err (ParseError :: new (format ! ("failed to parse {}: {}" , s , err))) , None => Err (err) , } }
};
}
