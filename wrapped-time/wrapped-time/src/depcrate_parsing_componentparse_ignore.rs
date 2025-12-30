// Generated macro for parse_ignore (function)
macro_rules! Depcrate_parsing_componentparse_ignore {
() => {
// Module: crate::parsing::component
// Provides: {"parse_ignore"}
// Dependencies: {}
# [doc = " Ignore the given number of bytes."] # [inline] pub (crate) fn parse_ignore (input : & [u8] , modifiers : modifier :: Ignore ,) -> Option < ParsedItem < '_ , () > > { let modifier :: Ignore { count } = modifiers ; let input = input . get ((count . get () . extend ()) ..) ? ; Some (ParsedItem (input , ())) }
};
}
