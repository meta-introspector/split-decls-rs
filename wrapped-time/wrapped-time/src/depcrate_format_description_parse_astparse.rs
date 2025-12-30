// Generated macro for parse (function)
macro_rules! Depcrate_format_description_parse_astparse {
() => {
// Module: crate::format_description::parse::ast
// Provides: {"parse"}
// Dependencies: {}
# [doc = " Parse the provided tokens into an AST."] # [inline] pub (super) fn parse < 'item : 'iter , 'iter , I : Iterator < Item = Result < lexer :: Token < 'item > , Error > > , const VERSION : usize , > (tokens : & 'iter mut lexer :: Lexed < I > ,) -> impl Iterator < Item = Result < Item < 'item > , Error > > + 'iter { validate_version ! (VERSION) ; parse_inner :: < _ , false , VERSION > (tokens) }
};
}
