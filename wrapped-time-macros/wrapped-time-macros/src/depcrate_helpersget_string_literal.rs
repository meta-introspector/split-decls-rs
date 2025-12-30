// Generated macro for get_string_literal (function)
macro_rules! Depcrate_helpersget_string_literal {
() => {
// Module: crate::helpers
// Provides: {"get_string_literal"}
// Dependencies: {}
# [cfg (any (feature = "formatting" , feature = "parsing"))] pub (crate) fn get_string_literal (mut tokens : impl Iterator < Item = TokenTree > ,) -> Result < (Span , Vec < u8 >) , Error > { match (tokens . next () , tokens . next ()) { (Some (TokenTree :: Literal (literal)) , None) => string :: parse (& literal) , (Some (tree) , None) => Err (Error :: ExpectedString { span_start : Some (tree . span ()) , span_end : Some (tree . span ()) , }) , (_ , Some (tree)) => Err (Error :: UnexpectedToken { tree }) , (None , None) => Err (Error :: ExpectedString { span_start : None , span_end : None , }) , } }
};
}
