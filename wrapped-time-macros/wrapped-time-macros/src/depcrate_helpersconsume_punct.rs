// Generated macro for consume_punct (function)
macro_rules! Depcrate_helpersconsume_punct {
() => {
// Module: crate::helpers
// Provides: {"consume_punct"}
// Dependencies: {}
pub (crate) fn consume_punct (c : char , chars : & mut Peekable < token_stream :: IntoIter > ,) -> Result < Span , Error > { match chars . peek () { Some (TokenTree :: Punct (punct)) if * punct == c => { let ret = Ok (punct . span ()) ; drop (chars . next ()) ; ret } Some (tree) => Err (Error :: UnexpectedToken { tree : tree . clone () }) , None => Err (Error :: UnexpectedEndOfInput) , } }
};
}
