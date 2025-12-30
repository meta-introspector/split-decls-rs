// Generated macro for days_in_year_month (function)
macro_rules! Depcrate_helpersdays_in_year_month {
() => {
// Module: crate::helpers
// Provides: {"days_in_year_month"}
// Dependencies: {}
pub (crate) fn days_in_year_month (year : i32 , month : u8) -> u8 { [31 , 28 , 31 , 30 , 31 , 30 , 31 , 31 , 30 , 31 , 30 , 31] [month . extend :: < usize > () - 1] + u8 :: from (month == 2 && is_leap_year (year)) }
};
}
