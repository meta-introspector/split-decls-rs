// Generated macro for parse_braces (function)
macro_rules! Depcrate_groupparse_braces {
() => {
// Module: crate::group
// Provides: {"parse_braces"}
// Dependencies: {}
# [doc (hidden)] pub fn parse_braces < 'a > (input : & ParseBuffer < 'a >) -> Result < Braces < 'a > > { parse_delimited (input , Delimiter :: Brace) . map (| (span , content) | Braces { token : token :: Brace (span) , content , }) }
};
}
