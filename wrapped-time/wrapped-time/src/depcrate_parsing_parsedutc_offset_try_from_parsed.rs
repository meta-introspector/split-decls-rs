// Generated macro for utc_offset_try_from_parsed (function)
macro_rules! Depcrate_parsing_parsedutc_offset_try_from_parsed {
() => {
// Module: crate::parsing::parsed
// Provides: {"utc_offset_try_from_parsed"}
// Dependencies: {}
# [inline] fn utc_offset_try_from_parsed < const REQUIRED : bool > (parsed : Parsed ,) -> Result < UtcOffset , error :: TryFromParsed > { let hour = match (REQUIRED , parsed . offset_hour ()) { (true , None) => return Err (InsufficientInformation) , (false , None) => return Ok (UtcOffset :: UTC) , (_ , Some (hour)) => hour , } ; let minute = parsed . offset_minute_signed () ; let second = minute . and_then (| _ | parsed . offset_second_signed ()) ; let minute = minute . unwrap_or (0) ; let second = second . unwrap_or (0) ; UtcOffset :: from_hms (hour , minute , second) . map_err (| mut err | { if err . name == "hours" { err . name = "offset hour" ; } else if err . name == "minutes" { err . name = "offset minute" ; } else if err . name == "seconds" { err . name = "offset second" ; } err . into () }) }
};
}
