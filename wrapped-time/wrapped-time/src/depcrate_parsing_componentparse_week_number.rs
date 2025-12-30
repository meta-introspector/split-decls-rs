// Generated macro for parse_week_number (function)
macro_rules! Depcrate_parsing_componentparse_week_number {
() => {
// Module: crate::parsing::component
// Provides: {"parse_week_number"}
// Dependencies: {}
# [doc = " Parse the \"week number\" component of a `Date`."] pub (crate) fn parse_week_number (input : & [u8] , modifiers : modifier :: WeekNumber ,) -> Option < ParsedItem < '_ , u8 > > { exactly_n_digits_padded :: < 2 , _ > (modifiers . padding) (input) }
};
}
