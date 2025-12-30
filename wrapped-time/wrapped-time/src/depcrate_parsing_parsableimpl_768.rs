// Generated macro for impl_768 (impl)
macro_rules! Depcrate_parsing_parsableimpl_768 {
() => {
// Module: crate::parsing::parsable
// Provides: {"impl_768"}
// Dependencies: {}
impl < const CONFIG : EncodedConfig > sealed :: Sealed for Iso8601 < CONFIG > { # [inline] fn parse_into < 'a > (& self , mut input : & 'a [u8] , parsed : & mut Parsed ,) -> Result < & 'a [u8] , error :: Parse > { use crate :: parsing :: combinator :: rfc :: iso8601 :: ExtendedKind ; let mut extended_kind = ExtendedKind :: Unknown ; let mut date_is_present = false ; let mut time_is_present = false ; let mut offset_is_present = false ; let mut first_error = None ; parsed . leap_second_allowed = true ; match Self :: parse_date (parsed , & mut extended_kind) (input) { Ok (new_input) => { input = new_input ; date_is_present = true ; } Err (err) => { first_error . get_or_insert (err) ; } } match Self :: parse_time (parsed , & mut extended_kind , date_is_present) (input) { Ok (new_input) => { input = new_input ; time_is_present = true ; } Err (err) => { first_error . get_or_insert (err) ; } } if ! date_is_present || time_is_present { match Self :: parse_offset (parsed , & mut extended_kind) (input) { Ok (new_input) => { input = new_input ; offset_is_present = true ; } Err (err) => { first_error . get_or_insert (err) ; } } } if ! date_is_present && ! time_is_present && ! offset_is_present { match first_error { Some (err) => return Err (err) , None => bug ! ("an error should be present if no components were parsed") , } } Ok (input) } }
};
}
