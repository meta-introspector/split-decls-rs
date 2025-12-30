// Generated macro for parse_with_version (function)
macro_rules! Depcrate_format_descriptionparse_with_version {
() => {
// Module: crate::format_description
// Provides: {"parse_with_version"}
// Dependencies: {}
pub (crate) fn parse_with_version (version : Option < crate :: FormatDescriptionVersion > , s : & [u8] , proc_span : proc_macro :: Span ,) -> Result < Vec < public :: OwnedFormatItem > , crate :: Error > { match version { Some (crate :: FormatDescriptionVersion :: V1) | None => parse :: < 1 > (s , proc_span) , Some (crate :: FormatDescriptionVersion :: V2) => parse :: < 2 > (s , proc_span) , } }
};
}
