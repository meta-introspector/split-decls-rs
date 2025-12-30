// Generated macro for parse_parens (function)
macro_rules! Depcrate_groupparse_parens {
() => {
// Module: crate::group
// Provides: {"parse_parens"}
// Dependencies: {}
# [doc (hidden)] pub fn parse_parens < 'a > (input : & ParseBuffer < 'a >) -> Result < Parens < 'a > > { parse_delimited (input , Delimiter :: Parenthesis) . map (| (span , content) | Parens { token : token :: Paren (span) , content , }) }
};
}
