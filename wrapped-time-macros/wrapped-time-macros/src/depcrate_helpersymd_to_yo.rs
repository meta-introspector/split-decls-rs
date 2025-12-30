// Generated macro for ymd_to_yo (function)
macro_rules! Depcrate_helpersymd_to_yo {
() => {
// Module: crate::helpers
// Provides: {"ymd_to_yo"}
// Dependencies: {}
pub (crate) fn ymd_to_yo (year : i32 , month : u8 , day : u8) -> (i32 , u16) { let ordinal = [0 , 31 , 59 , 90 , 120 , 151 , 181 , 212 , 243 , 273 , 304 , 334] [month . extend :: < usize > () - 1] + u16 :: from (month > 2 && is_leap_year (year)) ; (year , ordinal + u16 :: from (day)) }
};
}
