// Generated macro for parse_hour (function)
macro_rules! Depcrate_parsing_componentparse_hour {
() => {
// Module: crate::parsing::component
// Provides: {"parse_hour"}
// Dependencies: {}
# [doc = " Parse the \"hour\" component of a `Time`."] # [inline] pub (crate) fn parse_hour (input : & [u8] , modifiers : modifier :: Hour) -> Option < ParsedItem < '_ , u8 > > { exactly_n_digits_padded :: < 2 , _ > (modifiers . padding) (input) }
};
}
