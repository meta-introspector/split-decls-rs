// Generated macro for parse_macro_args (function)
macro_rules! Depcrate_parse_macrosparse_macro_args {
() => {
// Module: crate::parse::macros
// Provides: {"parse_macro_args"}
// Dependencies: {}
pub (crate) fn parse_macro_args (context : & RewriteContext < '_ > , tokens : TokenStream , style : Delimiter , forced_bracket : bool ,) -> Option < ParsedMacroArgs > { let mut parser = build_parser (context , tokens) ; let mut args = Vec :: new () ; let mut vec_with_semi = false ; let mut trailing_comma = false ; if Delimiter :: Brace != style { loop { if let Some (arg) = check_keyword (& mut parser) { args . push (arg) ; } else if let Some (arg) = parse_macro_arg (& mut parser) { args . push (arg) ; } else { return None ; } match parser . token . kind { TokenKind :: Eof => break , TokenKind :: Comma => () , TokenKind :: Semi => { if forced_bracket { parser . bump () ; if parser . token . kind != TokenKind :: Eof { match parse_macro_arg (& mut parser) { Some (arg) => { args . push (arg) ; parser . bump () ; if parser . token == TokenKind :: Eof && args . len () == 2 { vec_with_semi = true ; break ; } } None => { return None ; } } } } return None ; } _ if args . last () . map_or (false , MacroArg :: is_item) => continue , _ => return None , } parser . bump () ; if parser . token == TokenKind :: Eof { trailing_comma = true ; break ; } } } Some (ParsedMacroArgs { vec_with_semi , trailing_comma , args , }) }
};
}
