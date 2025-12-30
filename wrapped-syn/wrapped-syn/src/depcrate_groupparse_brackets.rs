// Generated macro for parse_brackets (function)
macro_rules! Depcrate_groupparse_brackets {
() => {
// Module: crate::group
// Provides: {"parse_brackets"}
// Dependencies: {}
# [doc (hidden)] pub fn parse_brackets < 'a > (input : & ParseBuffer < 'a >) -> Result < Brackets < 'a > > { parse_delimited (input , Delimiter :: Bracket) . map (| (span , content) | Brackets { token : token :: Bracket (span) , content , }) }
};
}
