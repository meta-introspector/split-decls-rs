// Generated macro for parse_second (function)
macro_rules! Depcrate_parsing_componentparse_second {
() => {
// Module: crate::parsing::component
// Provides: {"parse_second"}
// Dependencies: {}
# [doc = " Parse the \"second\" component of a `Time`."] # [inline] pub (crate) fn parse_second (input : & [u8] , modifiers : modifier :: Second ,) -> Option < ParsedItem < '_ , u8 > > { exactly_n_digits_padded :: < 2 , _ > (modifiers . padding) (input) }
};
}
