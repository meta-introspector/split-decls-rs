// Generated macro for parse_day (function)
macro_rules! Depcrate_parsing_componentparse_day {
() => {
// Module: crate::parsing::component
// Provides: {"parse_day"}
// Dependencies: {}
# [doc = " Parse the \"day\" component of a `Date`."] # [inline] pub (crate) fn parse_day (input : & [u8] , modifiers : modifier :: Day ,) -> Option < ParsedItem < '_ , NonZero < u8 > > > { exactly_n_digits_padded :: < 2 , _ > (modifiers . padding) (input) }
};
}
