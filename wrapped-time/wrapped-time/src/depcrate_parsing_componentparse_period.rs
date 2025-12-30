// Generated macro for parse_period (function)
macro_rules! Depcrate_parsing_componentparse_period {
() => {
// Module: crate::parsing::component
// Provides: {"parse_period"}
// Dependencies: {}
# [doc = " Parse the \"period\" component of a `Time`. Required if the hour is on a 12-hour clock."] # [inline] pub (crate) fn parse_period (input : & [u8] , modifiers : modifier :: Period ,) -> Option < ParsedItem < '_ , Period > > { first_match (if modifiers . is_uppercase { [(b"AM" . as_slice () , Period :: Am) , (b"PM" . as_slice () , Period :: Pm) ,] } else { [(b"am" . as_slice () , Period :: Am) , (b"pm" . as_slice () , Period :: Pm) ,] } , modifiers . case_sensitive ,) (input) }
};
}
