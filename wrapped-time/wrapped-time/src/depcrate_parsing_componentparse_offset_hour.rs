// Generated macro for parse_offset_hour (function)
macro_rules! Depcrate_parsing_componentparse_offset_hour {
() => {
// Module: crate::parsing::component
// Provides: {"parse_offset_hour"}
// Dependencies: {}
# [doc = " Parse the \"hour\" component of a `UtcOffset`."] # [doc = ""] # [doc = " Returns the value and whether the value is negative. This is used for when \"-0\" is parsed."] pub (crate) fn parse_offset_hour (input : & [u8] , modifiers : modifier :: OffsetHour ,) -> Option < ParsedItem < '_ , (i8 , bool) > > { let ParsedItem (input , sign) = opt (sign) (input) ; let ParsedItem (input , hour) = exactly_n_digits_padded :: < 2 , u8 > (modifiers . padding) (input) ? ; match sign { Some (b'-') => Some (ParsedItem (input , (- hour . cast_signed () , true))) , None if modifiers . sign_is_mandatory => None , _ => Some (ParsedItem (input , (hour . cast_signed () , false))) , } }
};
}
