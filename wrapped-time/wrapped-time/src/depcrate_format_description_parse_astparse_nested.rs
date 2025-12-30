// Generated macro for parse_nested (function)
macro_rules! Depcrate_format_description_parse_astparse_nested {
() => {
// Module: crate::format_description::parse::ast
// Provides: {"parse_nested"}
// Dependencies: {}
# [doc = " Parse a nested format description. The location provided is the most recent one consumed."] # [inline] fn parse_nested < 'a , I : Iterator < Item = Result < lexer :: Token < 'a > , Error > > , const VERSION : usize > (last_location : Location , tokens : & mut lexer :: Lexed < I > ,) -> Result < NestedFormatDescription < 'a > , Error > { validate_version ! (VERSION) ; let Some (opening_bracket) = tokens . next_if_opening_bracket () else { return Err (Error { _inner : unused (last_location . error ("expected opening bracket")) , public : crate :: error :: InvalidFormatDescription :: Expected { what : "opening bracket" , index : last_location . byte as usize , } , }) ; } ; let items = parse_inner :: < _ , true , VERSION > (tokens) . collect :: < Result < _ , _ > > () ? ; let Some (closing_bracket) = tokens . next_if_closing_bracket () else { return Err (Error { _inner : unused (opening_bracket . error ("unclosed bracket")) , public : crate :: error :: InvalidFormatDescription :: UnclosedOpeningBracket { index : opening_bracket . byte as usize , } , }) ; } ; let trailing_whitespace = tokens . next_if_whitespace () ; Ok (NestedFormatDescription { _opening_bracket : unused (opening_bracket) , items , _closing_bracket : unused (closing_bracket) , _trailing_whitespace : unused (trailing_whitespace) , }) }
};
}
