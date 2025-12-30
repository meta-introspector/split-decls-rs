// Generated macro for parse_unix_timestamp (function)
macro_rules! Depcrate_parsing_componentparse_unix_timestamp {
() => {
// Module: crate::parsing::component
// Provides: {"parse_unix_timestamp"}
// Dependencies: {}
# [doc = " Parse the Unix timestamp component."] pub (crate) fn parse_unix_timestamp (input : & [u8] , modifiers : modifier :: UnixTimestamp ,) -> Option < ParsedItem < '_ , i128 > > { let ParsedItem (input , sign) = opt (sign) (input) ; let ParsedItem (input , nano_timestamp) = match modifiers . precision { modifier :: UnixTimestampPrecision :: Second => { n_to_m_digits :: < 1 , 14 , u128 > (input) ? . map (| val | val * Nanosecond :: per_t :: < u128 > (Second)) } modifier :: UnixTimestampPrecision :: Millisecond => n_to_m_digits :: < 1 , 17 , u128 > (input) ? . map (| val | val * Nanosecond :: per_t :: < u128 > (Millisecond)) , modifier :: UnixTimestampPrecision :: Microsecond => n_to_m_digits :: < 1 , 20 , u128 > (input) ? . map (| val | val * Nanosecond :: per_t :: < u128 > (Microsecond)) , modifier :: UnixTimestampPrecision :: Nanosecond => n_to_m_digits :: < 1 , 23 , _ > (input) ? , } ; match sign { Some (b'-') => Some (ParsedItem (input , - nano_timestamp . cast_signed ())) , None if modifiers . sign_is_mandatory => None , _ => Some (ParsedItem (input , nano_timestamp . cast_signed ())) , } }
};
}
