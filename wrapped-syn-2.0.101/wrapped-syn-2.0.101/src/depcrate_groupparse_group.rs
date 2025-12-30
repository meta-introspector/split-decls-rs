// Generated macro for parse_group (function)
macro_rules! Depcrate_groupparse_group {
() => {
// Module: crate::group
// Provides: {"parse_group"}
// Dependencies: {}
# [cfg (any (feature = "full" , feature = "derive"))] pub (crate) fn parse_group < 'a > (input : & ParseBuffer < 'a >) -> Result < Group < 'a > > { parse_delimited (input , Delimiter :: None) . map (| (span , content) | Group { token : token :: Group (span . join ()) , content , }) }
};
}
