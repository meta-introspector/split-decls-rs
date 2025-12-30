// Generated macro for peek_sus_token (function)
macro_rules! Depcrate_parser_commandpeek_sus_token {
() => {
// Module: crate::parser::command
// Provides: {"peek_sus_token"}
// Dependencies: {}
fn peek_sus_token (span : Span) -> ParseResult < () > { alt ((swallow (peek (alt ((token (Keyword :: Do) , token (Keyword :: Done) , token (Keyword :: Else) , token (Keyword :: Esac) , token (Keyword :: Elif) , token (Keyword :: Fi) , token (Keyword :: Then) ,)))) , swallow (peek (one_of ("})"))) , swallow (peek (token (ControlOp :: DSemi))) ,)) (span) }
};
}
