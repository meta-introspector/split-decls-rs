// Generated macro for parse_offset_minute (function)
macro_rules! Depcrate_parsing_componentparse_offset_minute {
() => {
// Module: crate::parsing::component
// Provides: {"parse_offset_minute"}
// Dependencies: {}
# [doc = " Parse the \"minute\" component of a `UtcOffset`."] # [inline] pub (crate) fn parse_offset_minute (input : & [u8] , modifiers : modifier :: OffsetMinute ,) -> Option < ParsedItem < '_ , i8 > > { Some (exactly_n_digits_padded :: < 2 , u8 > (modifiers . padding) (input) ? . map (| offset_minute | offset_minute . cast_signed ()) ,) }
};
}
