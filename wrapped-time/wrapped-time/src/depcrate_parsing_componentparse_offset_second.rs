// Generated macro for parse_offset_second (function)
macro_rules! Depcrate_parsing_componentparse_offset_second {
() => {
// Module: crate::parsing::component
// Provides: {"parse_offset_second"}
// Dependencies: {}
# [doc = " Parse the \"second\" component of a `UtcOffset`."] # [inline] pub (crate) fn parse_offset_second (input : & [u8] , modifiers : modifier :: OffsetSecond ,) -> Option < ParsedItem < '_ , i8 > > { Some (exactly_n_digits_padded :: < 2 , u8 > (modifiers . padding) (input) ? . map (| offset_second | offset_second . cast_signed ()) ,) }
};
}
