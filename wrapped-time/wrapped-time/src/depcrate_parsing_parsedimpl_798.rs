// Generated macro for impl_798 (impl)
macro_rules! Depcrate_parsing_parsedimpl_798 {
() => {
// Module: crate::parsing::parsed
// Provides: {"impl_798"}
// Dependencies: {}
impl TryFrom < Parsed > for OffsetDateTime { type Error = error :: TryFromParsed ; # [inline] fn try_from (mut parsed : Parsed) -> Result < Self , Self :: Error > { if let Some (timestamp) = parsed . unix_timestamp_nanos () { let mut value = Self :: from_unix_timestamp_nanos (timestamp) ? ; if let Some (subsecond) = parsed . subsecond () { value = value . replace_nanosecond (subsecond) ? ; } return Ok (value) ; } let leap_second_input = if parsed . leap_second_allowed && parsed . second () == Some (60) { if parsed . set_second (59) . is_none () { bug ! ("59 is a valid second") ; } if parsed . set_subsecond (999_999_999) . is_none () { bug ! ("999_999_999 is a valid subsecond") ; } true } else { false } ; let dt = Self :: new_in_offset (Date :: try_from (parsed) ? , Time :: try_from (parsed) ? , UtcOffset :: try_from (parsed) ? ,) ; if leap_second_input && ! dt . is_valid_leap_second_stand_in () { return Err (error :: TryFromParsed :: ComponentRange (error :: ComponentRange { name : "second" , minimum : 0 , maximum : 59 , value : 60 , conditional_message : Some ("because leap seconds are not supported") , } ,)) ; } Ok (dt) } }
};
}
