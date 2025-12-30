// Generated macro for ywd_to_yo (function)
macro_rules! Depcrate_helpersywd_to_yo {
() => {
// Module: crate::helpers
// Provides: {"ywd_to_yo"}
// Dependencies: {}
pub (crate) fn ywd_to_yo (year : i32 , week : u8 , iso_weekday_number : u8) -> (i32 , u16) { let (ordinal , overflow) = (u16 :: from (week) * 7 + u16 :: from (iso_weekday_number)) . overflowing_sub (u16 :: from (jan_weekday (year , 4)) + 4) ; if overflow || ordinal == 0 { return (year - 1 , (ordinal . wrapping_add (days_in_year (year - 1)))) ; } let days_in_cur_year = days_in_year (year) ; if ordinal > days_in_cur_year { (year + 1 , ordinal - days_in_cur_year) } else { (year , ordinal) } }
};
}
