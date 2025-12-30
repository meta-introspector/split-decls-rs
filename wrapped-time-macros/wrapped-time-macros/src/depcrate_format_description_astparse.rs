// Generated macro for parse (function)
macro_rules! Depcrate_format_description_astparse {
() => {
// Module: crate::format_description::ast
// Provides: {"parse"}
// Dependencies: {}
pub (super) fn parse < 'item : 'iter , 'iter , I : Iterator < Item = Result < lexer :: Token < 'item > , Error > > , const VERSION : u8 , > (tokens : & 'iter mut lexer :: Lexed < I > ,) -> impl Iterator < Item = Result < Item < 'item > , Error > > + 'iter { assert ! (version ! (1 ..= 2)) ; parse_inner :: < _ , false , VERSION > (tokens) }
};
}
