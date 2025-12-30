// Generated macro for parse_nested (function)
macro_rules! Depcrate_format_description_astparse_nested {
() => {
// Module: crate::format_description::ast
// Provides: {"parse_nested"}
// Dependencies: {}
fn parse_nested < 'a , I : Iterator < Item = Result < lexer :: Token < 'a > , Error > > , const VERSION : u8 > (last_location : Location , tokens : & mut lexer :: Lexed < I > ,) -> Result < NestedFormatDescription < 'a > , Error > { let Some (opening_bracket) = tokens . next_if_opening_bracket () else { return Err (last_location . error ("expected opening bracket")) ; } ; let items = parse_inner :: < _ , true , VERSION > (tokens) . collect :: < Result < _ , _ > > () ? ; let Some (closing_bracket) = tokens . next_if_closing_bracket () else { return Err (opening_bracket . error ("unclosed bracket")) ; } ; let trailing_whitespace = tokens . next_if_whitespace () ; Ok (NestedFormatDescription { _opening_bracket : unused (opening_bracket) , items , _closing_bracket : unused (closing_bracket) , _trailing_whitespace : unused (trailing_whitespace) , }) }
};
}
