// Generated macro for parse_end (function)
macro_rules! Depcrate_parsing_componentparse_end {
() => {
// Module: crate::parsing::component
// Provides: {"parse_end"}
// Dependencies: {}
# [doc = " Parse the `end` component, which represents the end of input. If any input is remaining, `None`"] # [doc = " is returned."] # [inline] pub (crate) const fn parse_end (input : & [u8] , end : modifier :: End) -> Option < ParsedItem < '_ , () > > { let modifier :: End { } = end ; if input . is_empty () { Some (ParsedItem (input , ())) } else { None } }
};
}
