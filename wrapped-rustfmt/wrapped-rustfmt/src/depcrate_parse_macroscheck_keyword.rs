// Generated macro for check_keyword (function)
macro_rules! Depcrate_parse_macroscheck_keyword {
() => {
// Module: crate::parse::macros
// Provides: {"check_keyword"}
// Dependencies: {}
fn check_keyword < 'a , 'b : 'a > (parser : & 'a mut Parser < 'b >) -> Option < MacroArg > { if parser . token . is_reserved_ident () && parser . look_ahead (1 , | t | * t == TokenKind :: Eof || * t == TokenKind :: Comma) { let keyword = parser . token . ident () . unwrap () . 0 . name ; parser . bump () ; Some (MacroArg :: Keyword (symbol :: Ident :: with_dummy_span (keyword) , parser . prev_token . span ,)) } else { None } }
};
}
