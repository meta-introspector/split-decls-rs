// Generated macro for parse_macro_delimiter (function)
macro_rules! Depcrate_helpersparse_macro_delimiter {
() => {
// Module: crate::helpers
// Provides: {"parse_macro_delimiter"}
// Dependencies: {}
pub fn parse_macro_delimiter < 'a > (input : & ParseBuffer < 'a > ,) -> Result < (MacroDelimiter , ParseBuffer < 'a >) > { let content ; let token = if input . peek (token :: Paren) { MacroDelimiter :: Paren (parenthesized ! (content in input)) } else if input . peek (token :: Bracket) { MacroDelimiter :: Bracket (bracketed ! (content in input)) } else if input . peek (token :: Brace) { MacroDelimiter :: Brace (braced ! (content in input)) } else { return Err (input . error ("expected `(`, `[` or `{`")) ; } ; Ok ((token , content)) }
};
}
