// Generated macro for consume_any_ident (function)
macro_rules! Depcrate_helpersconsume_any_ident {
() => {
// Module: crate::helpers
// Provides: {"consume_any_ident"}
// Dependencies: {}
pub (crate) fn consume_any_ident (idents : & [& str] , chars : & mut Peekable < token_stream :: IntoIter > ,) -> Result < Span , Error > { match chars . peek () { Some (TokenTree :: Ident (char)) if idents . contains (& char . to_string () . as_str ()) => { let ret = Ok (char . span ()) ; drop (chars . next ()) ; ret } Some (tree) => Err (Error :: UnexpectedToken { tree : tree . clone () }) , None => Err (Error :: UnexpectedEndOfInput) , } }
};
}
