// Generated macro for parse_minute (function)
macro_rules! Depcrate_parsing_componentparse_minute {
() => {
// Module: crate::parsing::component
// Provides: {"parse_minute"}
// Dependencies: {}
# [doc = " Parse the \"minute\" component of a `Time`."] # [inline] pub (crate) fn parse_minute (input : & [u8] , modifiers : modifier :: Minute ,) -> Option < ParsedItem < '_ , u8 > > { exactly_n_digits_padded :: < 2 , _ > (modifiers . padding) (input) }
};
}
