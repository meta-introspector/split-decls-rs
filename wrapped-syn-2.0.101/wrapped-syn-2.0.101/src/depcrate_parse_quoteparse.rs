// Generated macro for parse (function)
macro_rules! Depcrate_parse_quoteparse {
() => {
// Module: crate::parse_quote
// Provides: {"parse"}
// Dependencies: {}
# [doc (hidden)] # [track_caller] pub fn parse < T : ParseQuote > (token_stream : TokenStream) -> T { let parser = T :: parse ; match parser . parse2 (token_stream) { Ok (t) => t , Err (err) => panic ! ("{}" , err) , } }
};
}
